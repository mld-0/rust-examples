//  vim-modelines: {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2

//use std::fmt::Write;      //  wrong 'Write'

use std::io::Write;
use std::io::BufWriter;

#[allow(unused_imports)]
use clap::ArgMatches;

#[allow(unused_imports)]
use std::fs::File;

#[allow(unused_imports)]
use std::path::Path;


//  Notes:
//  2023-06-11T23:00:34AEST whatever the solution for Writer -> does it require a mutable instance of 'Printer' to be used? Is there a solution (beyond the most simple having a filepath and opening it each function call?)
//  2023-06-11T23:19:33AEST 'Box<dyn Write>' implies something mutable?
//  2023-06-12T00:11:01AEST if we give 'Printer' a buffer by-value, getting it back is a serious headache
//  2023-06-12T00:12:07AEST I'm going to guess designing a class which can take 'Write' either by mut reference *or* by value is *not* [{a sensible approach}]
//  2023-06-12T00:13:41AEST returning an owned 'Option' is what '.take()' is for
//  2023-06-12T00:22:46AEST creating a `Printer1` with/without casting to `&mut dyn Write` -> does it produce the same assembly (is it being case implicitly anyway?)
//  2023-06-12T00:29:26AEST behaviour for filepath example for an existing file? (will truncate the existing file to zero length, effectively clearing its contents -> if the file was long, might any old contents be left?)
//  2023-06-12T00:35:06AEST what types can clap::ArgMatches be?
//  2023-06-12T00:35:51AEST what is `std::fmt::Write` used for? (this being what we were incorrectly  initially trying to use)
//  2023-06-12T00:41:19AEST writing to a file with our `out!()` macro (calls `println!()` -> there is no trailing in the file?)


macro_rules! out {
    ($opt:expr, $fmt:expr) => {
        match $opt {
            Some(ref mut out) => writeln!(out, $fmt).unwrap(),
            None => println!($fmt),
        }
    };
    ($opt:expr, $fmt:expr, $($arg:tt)*) => {
        match $opt {
            Some(ref mut out) => writeln!(out, $fmt, $($arg)*).unwrap(),
            None => println!($fmt, $($arg)*),
        }
    };
}


pub struct Printer1<'a> {
    output: Option<&'a mut dyn Write>,
}

impl<'a> Printer1<'a> {
    pub fn new(output: Option<&'a mut dyn Write>) -> Printer1 {
        Printer1 { output }
    }

    pub fn new_by_ref(output: &'a mut dyn Write) -> Printer1 {
        Printer1 { output: Some(output) }
    }

    pub fn default() -> Printer1<'a> {
        Printer1 { output: None }
    }

    pub fn write(&mut self, s: &str) {
        out!(self.output, "{}", s);
    }

    //  ctor using &ArgMatches?
    //  <>

}


//  Attempting to store 'Write' object by-value
//  {{{
//pub struct Printer2 {
//    output: Option<Box<dyn Write>>,
//}
//
//impl Printer2 {
//
//    pub fn write(&mut self, s: &str) {
//        out!(self.output, "{}", s);
//    }
//
//    pub fn default() -> Printer2 {
//        Printer2 { output: None }
//    }
//
//    //  Using this approach, need to pass-out any in-memory buffer when we are finished with it
//    //  (which gets nasty ... see 'Printer3'
//    pub fn get_buffer(&mut self) -> Option<Box<dyn Write>> {
//        self.output.take()
//    }
//
//}
//
//
//use std::any::Any;
//pub trait WriteAny: Write + Any {}
//impl<T: Write + Any + 'static> WriteAny for T {}
//pub struct Printer3 {
//    output: Option<Box<dyn WriteAny>>,
//}
//
//impl Printer3 {
//
//    pub fn write(&mut self, s: &str) {
//        out!(self.output, "{}", s);
//    }
//
//    pub fn default() -> Printer3 {
//        Printer3 { output: None }
//    }
//
//    pub fn new_from_path(filepath: &str) -> Printer3 {
//        unimplemented!();
//    }
//
//    //  Using this approach, need to pass-out any in-memory buffer when we are finished with it
//    pub fn get_buffer(&mut self) -> Option<Box<dyn WriteAny>> {
//        self.output.take()
//    }
//
//}
//  }}}

#[allow(non_snake_case)]
fn pass_Write_by_val() 
{
    //  Attempting to pass buffer by value
    //  (which works - the nightmare is getting it back again)
    //let b1 = Vec::<u8>::new();
    //let mut p1 = Printer3 { output: Some(Box::new(b1)) };
    //p1.write("Line One");
    ////let b1 = (*p1.get_buffer()).downcast::<Vec<u8>>;
    ////println!("{:?}", b1);

}

#[allow(non_snake_case)]
fn pass_Write_by_mut_ref()
{
    //  Write to buffer
    let mut b1 = Vec::<u8>::new();
    let mut p1 = Printer1 { output: Some(&mut b1) };
    //let mut p1 = Printer1 { output: Some(&mut b1 as &mut dyn Write) };  //  unnecessary?
    p1.write("Line One");
    let s1 = String::from_utf8(b1.clone()).unwrap();
    print!("{}", s1);
    println!("{:?}", b1);


    //  Write to stdout
    let mut p2 = Printer1::default();
    p2.write("Line Two");


    //  Write to file
    let filepath = "/tmp/temp.rust-examples-box-dyn-write.txt";
    let mut f = BufWriter::new(File::create(filepath).expect("File creation failed"));
    let mut p3 = Printer1::new(Some(&mut f));
    p3.write("Line Three");
    p3.write("Line Four");
    drop(p3);   //  unnecessary
    f.flush().expect("Failed to flush BufWriter");
    drop(f);    //  unnecessary


    let mut b4 = Vec::<u8>::new();
    let mut p4 = Printer1::new_by_ref(&mut b4);
    p4.write("Line Five");
    let s4 = String::from_utf8_lossy(&b4);
    print!("{}", s4);
    println!("{:?}", b4);


    let mut b5 = Vec::<u8>::new();
    let mut p5 = Printer1::new(Some(&mut b5));
    p5.write("Line Six");
    let s5 = String::from_utf8(b5).unwrap();
    print!("{}", s5);


    //  Append to file
    //use std::fs::OpenOptions;
    //let file = OpenOptions::new().append(true).create(true).open(filepath)?;
    //  <>

}


fn main() 
{
    pass_Write_by_val();
    pass_Write_by_mut_ref();
}


//  {{{
//pub fn new(arg_matches: &ArgMatches) -> Printer_ii {
//    if let Some(filepath) = arg_matches.value_of("output") {
//        //  {{{
//        //let file = File::create(Path::new(file_path)).expect("Failed to create `matches` file 'output'");
//        //Printer_ii { output: Some( Box::new(BufWriter::new(file)) ) }
//        //let file = File::create(filepath).expect("Failed to create 'filepath'");
//        //let buffered_file = BufWriter::new(file);
//        //let writer: Box<dyn Write> = Box::new(buffered_file);
//        //Printer_ii { output: Some(writer) }
//        //  }}}
//        Printer_ii::default()
//    } else {
//        Printer_ii::default()
//    }
//}
//  }}}

