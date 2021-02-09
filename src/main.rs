/*
 * A Rust implementation of Dynamic Markov Compression [0].
 *
 * https://github.com/oscar-franzen/dynamic-markov-compression
 *
 * Feedback: Oscar Franzen; <p.oscar.franzen@gmail.com>
 *
 * [0] https://en.wikipedia.org/wiki/Dynamic_Markov_compression
 *
 */

use std::fs::File;
use std::env;
use std::process;
use std::io::{BufReader, Read, Write};

use getopts::Options;
use array2d::Array2D;

const VERSION : &str = "0.1";
const BLOCK_SIZE : usize = 1_048_576;
//const MAX_NODES : usize = 1000000;
//const MAX_NODES : usize = 524269;

const THRESHOLD : f32 = 2.0;
const BIGTHRES : f32 = 2.0;

fn help() {
    println!("Usage: dmc [OPTION] [FILE]\n");
    println!(" -c, --compress          compress a file");
    println!(" -d, --decompress        decompress a file");
    println!(" -o, --output [STRING]   write output to a specific file");
    println!(" -n, --nodes [INTEGER]   maximum number of nodes to use");
    println!("                         (default is 524269)");
    println!(" -v, --verbose           verbose mode");
    println!(" -V, --version           version number");
    println!(" -h, --help              this help");
    println!("\n");
    println!("DMC is Dynamic Markov Compression, a lossless data compression");
    println!("algorithm invented by Cormack and Horspool in 1987.");
    process::exit(1);
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut verbose = false;
    let mut output = String::new();
    let mut n_nodes : usize;

    if args.len() <= 1 {
	help();
    }

    let mut opts = Options::new();
    opts.optflag("c", "compress", "");
    opts.optflag("d", "decompress", "");
    opts.optflag("h", "help", "");
    opts.optflag("V", "version", "");
    opts.optflag("v", "verbose", "");

    opts.optopt("o", "output", "", "NAME");
    opts.optopt("n", "nodes", "", "NODES");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
	help();
    }

    if matches.opt_present("o") {
	output = matches.opt_str("o").unwrap();
    }
    
    if matches.opt_present("v") {
	verbose = true;
    }

    if matches.opt_present("n") {
	n_nodes = matches.opt_str("n").unwrap().parse().unwrap();
    } else {
	n_nodes = 524269;
    }

    if verbose {
	eprintln!("Using {} nodes", n_nodes);
    }

    if matches.opt_present("V") {
	println!("DMC in Rust. Version: {}.\n\
https://github.com/oscar-franzen/dynamic-markov-compression\n\
Feedback: <p.oscar.franzen@gmail.com>", VERSION);
	process::exit(0);
    }

    let file = &args[args.len()-1]; // input file

    if verbose {
	eprintln!("Loading {}", file);
    }

    if matches.opt_present("c") {
	if output == "" {
	    output = format!("{}.{}", file, "dmc");
	}
	
	compress(&file, &output, n_nodes, verbose);
    }

    if matches.opt_present("d") {
	if output == "" {
	    output = format!("{}.{}", file, "decomp");
	}
	decompress(&file, &output, n_nodes, verbose);
    } 
}

#[derive(Clone, Copy)]
struct Node {
    count : [f32; 3],
    _ptr_next : [*mut Node; 3],
}

impl Node {
    fn new() -> Node {
	Node {
	    count: [0.0, 0.0, 0.0],
	    _ptr_next : [std::ptr::null_mut(); 3]
	}
    }
}

fn predict(mut _p : *mut Node) -> f32 {
    unsafe {
	(*_p).count[0] / ((*_p).count[0] + (*_p).count[1])
    }
}

fn pupdate(mut nodes : &mut Array2D<Node>,
	   mut _p : *mut Node,
	   mut predictor : &mut Vec<Node>,
	   mut pred_idx : &mut usize,
	   b : usize,
	   n_nodes : usize,
	   verbose : bool) -> *mut Node {


    unsafe {
	let mut r : f32;
	let sum : f32 = (*(*_p)._ptr_next[b]).count[0]+(*(*_p)._ptr_next[b]).count[1];
	
	if (*_p).count[b] >= THRESHOLD && sum >= (BIGTHRES + (*_p).count[b]) {
	    
	    r = (*_p).count[b]/sum;

	    let mut new : *mut Node;
	    new = &mut predictor[*pred_idx];

	    (*new).count[0] = (*(*_p)._ptr_next[b]).count[0] * r;
	    (*(*_p)._ptr_next[b]).count[0] -= (*new).count[0];

	    (*new).count[1] = (*(*_p)._ptr_next[b]).count[1] * r;
	    (*(*_p)._ptr_next[b]).count[1] -= (*new).count[1];

	    (*new)._ptr_next[0] = (*(*_p)._ptr_next[b])._ptr_next[0];
	    (*new)._ptr_next[1] = (*(*_p)._ptr_next[b])._ptr_next[1];

	    (*_p)._ptr_next[b] = new;
	
	    *pred_idx += 1;
	}

	(*_p).count[b] += 1.0;

	let mut qwe : *mut Node = (*_p)._ptr_next[b] as *mut Node;

	_p = qwe as *mut Node;
	
	if *pred_idx >= n_nodes {
	    if verbose {
		println!("flushing...");
	    }
	    
	    _p = pflush(&mut nodes, &mut pred_idx, _p);
	}

	return _p;
    }
}

fn pflush(mut nodes : &mut Array2D<Node>,
	  mut pred_idx : &mut usize,
	  mut _p : *mut Node) -> *mut Node {
    
    for j in 0..256 {
     	for i in 0..127 {
	    nodes[(j, i)].count[0] = 0.2;
	    nodes[(j, i)].count[1] = 0.2;

	    nodes[(j, i)]._ptr_next[0] = &mut nodes[(j, 2*i+1)];
	    nodes[(j, i)]._ptr_next[1] = &mut nodes[(j, 2*i+2)];
     	}

	for i in 127..255 {
	    nodes[(j, i)].count[0] = 0.2;
	    nodes[(j, i)].count[1] = 0.2;

	    nodes[(j, i)]._ptr_next[0] = &mut nodes[(i+1, 0)];
	    nodes[(j, i)]._ptr_next[1] = &mut nodes[(i-127, 0)];
	}
    }

    *pred_idx = 0;
    // the below does not work. is there a valid way of doing this
    // except using unsafe and returning?
    
    //_p = &mut nodes[(0, 0)];

    unsafe {
	let mut qwe : *mut Node = &mut nodes[(0,0)];
	_p = qwe as *mut Node;
	return _p;
    }
}

fn compress(input : &str,
	    output : &str,
	    n_nodes : usize,
	    verbose : bool) {
    let mut fh_out = File::create(&output).unwrap();

    let mut nodes = Array2D::filled_with(Node::new(), 256, 256);
    let fh_in = File::open(&input);

    let mut reader = BufReader::new(fh_in.unwrap());
    let mut buf = [0; BLOCK_SIZE];

    let mut bit : i32;
    let mut mid : i32;
    let mut min : i32 = 0;
    let mut max : i32 = 0x1000000;

    let mut _p : *mut Node = &mut nodes[(0, 0)];
    
    let mut new : (usize, usize) = (0, 0);
    let mut nodebuf : (usize, usize) = (0, 0);

    let mut predictor = vec![Node::new(); n_nodes];
    let mut pred_idx : usize = 0;

    let mut outbytes : u32 = 3;
    let mut inbytes : u32 = 0;
    let mut pout : u32 = 3;

    let mut test = 0;
    let mut total_bytes_read = 0;

    _p = pflush(&mut nodes, &mut pred_idx, _p);
        
    loop {
        let res = reader.read(&mut buf).unwrap();
	if verbose {
	    eprintln!("Read {} bytes", res);
	}

        if res == 0 {
	    min = max - 1;
            break;
        }

	total_bytes_read += res;
	
	for i in 0..res {
	    let b : u8 = buf[i];
	    test += 1;
	    
	    for k in 0..8 {
		bit = ((b as i32) << k) & 0x80;
		mid = (min as f32 + (max-min-1) as f32 * predict(_p)) as i32;

		_p = pupdate(&mut nodes,
			     _p,
			     &mut predictor,
			     &mut pred_idx,
			     (bit != 0) as usize,
			     n_nodes,
			     verbose);

		if mid == min {
		    mid += 1;
		}

		if mid == (max-1) {
		    mid -= 1;
		}

		if bit != 0 {
		    min = mid;
		} else {
		    max = mid;
		}
		
		while (max-min) < 256 {
		    if bit != 0 {
			max -= 1;
		    }
		    
		    fh_out.write_all( &[(min >> 16) as u8] );
		    outbytes += 1;

		    min = (min << 8) & 0xffff00;
		    max = ((max << 8) & 0xffff00 );

		    if min >= max {
			max = 0x1000000;
		    }
		}
	    }

	    inbytes += 1;

	    if (inbytes & 0xff) == 0 {
		if (inbytes & 0xffff) == 0 && verbose {
		    eprintln!("compressing... bytes in {}, \
			       bytes out {}, ratio {}\r", inbytes, outbytes, (outbytes as f32/inbytes as f32) );
		}

		if (outbytes - pout) > 256 {
		    _p = pflush(&mut nodes, &mut pred_idx, _p);
		}
		
		pout = outbytes;
	    }
	}
    }

    fh_out.write_all( &[(min >> 16) as u8] );
    fh_out.write_all( &[((min >> 8) & 0xff) as u8] );
    fh_out.write_all( &[(min & 0x00ff) as u8] );

    if verbose {
	eprintln!("Read {} bytes in total.", total_bytes_read);
    }
}

fn decompress(input : &str,
	      output : &str,
	      n_nodes : usize,
	      verbose : bool) {
    let mut fh_out = File::create(&output).unwrap();
    let fh_in = File::open(&input);
    let mut reader = BufReader::new(fh_in.unwrap());
    
    let mut bit : i32;
    let mut c : i32;
    let mut val : i32;
    let mut mid : i32;
    let mut min : i32 = 0;
    let mut max : i32 = 0x1000000;
    let mut outbytes : u32 = 0;
    let mut inbytes : u32 = 3;
    let mut pin : u32 = 3;
    let mut test = 0;

    let mut nodes = Array2D::filled_with(Node::new(), 256, 256);
    let mut _p : *mut Node = &mut nodes[(0, 0)];

    let mut predictor = vec![Node::new(); n_nodes];
    let mut pred_idx : usize = 0;
    
    _p = pflush(&mut nodes, &mut pred_idx, _p);
    
    let mut buf = [0; 3];
    if reader.read(&mut buf).unwrap() != 3 {
	eprintln!("error: read wrong number of bytes");
	process::exit(1);
    }

    val = ((buf[0] as i32) << 16);
    val += ((buf[1] as i32) << 8);
    val += buf[2] as i32;

    let mut end : bool = false;
           
    loop {
	c = 0;

	if val == (max-1) {
	    if verbose {
		eprintln!("decompression done.");
	    }
	    
	    break;
	}

	for k in 0..8 {
	    mid = (min as f32 + (max-min-1) as f32 * predict(_p)) as i32;

	    if mid == min {
		mid += 1;
	    }
		
	    if mid == (max-1) {
		mid -= 1;
	    }
				   
	    if val >= mid {
		bit = 1;
		min = mid;
	    } else {
		bit = 0;
		max = mid;
	    }
	    
	    _p = pupdate(&mut nodes,
			 _p,
			 &mut predictor,
			 &mut pred_idx,	
			 (bit != 0) as usize,
			 n_nodes,
			 verbose);
	    
	    c = c + c + bit;

	    while (max-min) < 256 {
		if bit != 0 {
		    max -= 1;
		}
		
		inbytes += 1;
		
		let mut b = [0; 1];
		let res = reader.read(&mut b).unwrap();

		if res == 0 {
		    end = true;
		    break;
		}
		
		let b : u8 = b[0] as u8;
		
		val = ((val << 8) & 0xffff00 | (b & 0xff) as i32);
		min = (min << 8) & 0xffff00;
		max = ((max << 8) & 0xffff00);

		if min >= max {
		    max = 0x1000000;
		}	    
	    }
	}

	if end == false {
	    fh_out.write_all( &[c as u8] );
	}

	outbytes += 1;
	    
	if (outbytes & 0xff) == 0 {
	    if (inbytes - pin) > 256 {
		_p = pflush(&mut nodes, &mut pred_idx, _p);
	    }
		
	    pin = inbytes;
	}
    }
}
