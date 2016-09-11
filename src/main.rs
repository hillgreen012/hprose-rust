/**********************************************************\
|                                                          |
|                          hprose                          |
|                                                          |
| Official WebSite: http://www.hprose.com/                 |
|                   http://www.hprose.org/                 |
|                                                          |
\**********************************************************/
/**********************************************************\
 *                                                        *
 * main.rs                                                *
 *                                                        *
 * hprose main for Rust.                                  *
 *                                                        *
 * LastModified: Sep 11, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

#![feature(test)]

mod io;

use io::writer::Writer;

fn main() {
    let mut writer = Writer::new(true);
    writer
        .serialize(&true)
        .serialize(&false)
        .serialize(&8)
        .serialize(&std::f32::consts::PI)
        .serialize(&std::f64::consts::PI)
        .serialize("你好,hello!")
    ;
    println!("{}", writer.string());
    
    let mut writer2 = Writer::new(false);
    let v: Vec<i32> = vec![2, 3];
    writer2
        .serialize(&v)
        .serialize(&v)
    ;
    println!("{}", writer2.string());
}