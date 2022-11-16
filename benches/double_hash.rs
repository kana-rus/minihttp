#![feature(test)]
extern crate test;

use test::Bencher;
use std::collections::HashMap;


#[derive(PartialEq, Eq, Hash)]
enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
    CONNECT,
}

fn sample_1() {
    println!("Hello, world! -- from 1")
}
fn sample_2() {
    println!("Hello, world! -- from 2")
}
fn sample_3() {
    println!("Hello, world! -- from 3")
}
fn sample_4() {
    println!("Hello, world! -- from 4")
}
fn sample_5() {
    println!("Hello, world! -- from 5")
}
fn sample_6() {
    println!("Hello, world! -- from 6")
}
fn sample_7() {
    println!("Hello, world! -- from 7")
}
fn sample_8() {
    println!("Hello, world! -- from 8")
}

const CASES: [(Method, &str); 10] = [
    (Method::CONNECT, "/"),
    (Method::DELETE, "/"),
    (Method::GET, "path1"),
    (Method::PATCH, "path2"),
    (Method::PATCH, "/"),
    (Method::POST, "/path2"),
    (Method::DELETE, "/path1"),
    (Method::POST, "path3"),
    (Method::POST, "/path3"),
    (Method::PATCH, "/path2"),
];


#[bench]  // 8,852 ns/iter (+/- 114); 2022-11-16
fn double_hash_with_tupple(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..10 {
            let map = HashMap::from([
                ((Method::GET,     "/"     ), sample_1 as fn()),
                ((Method::POST,    "/"     ), sample_2 as fn()),
                ((Method::CONNECT, "/"     ), sample_3 as fn()),
                ((Method::GET,     "/path1"), sample_4 as fn()),
                ((Method::POST,    "/path1"), sample_5 as fn()),
                ((Method::CONNECT, "/path1"), sample_6 as fn()),
                ((Method::DELETE,  "/path1"), sample_7 as fn()),
                ((Method::GET,     "/path2"), sample_4 as fn()),
                ((Method::POST,    "/path2"), sample_5 as fn()),
                ((Method::CONNECT, "/path2"), sample_6 as fn()),
                ((Method::DELETE,  "/path2"), sample_7 as fn()),
                ((Method::PATCH,   "/path2"), sample_8 as fn()),
                ((Method::POST,    "/path3"), sample_5 as fn()),
                ((Method::DELETE,  "/path3"), sample_7 as fn()),
                ((Method::GET,     "/path3"), sample_4 as fn()),
                ((Method::CONNECT, "/path3"), sample_3 as fn()),
                ((Method::PATCH,   "/path3"), sample_1 as fn()),
            ]);
            for method_and_path in &CASES {
                let Some(handler) = map.get(method_and_path) else {
                    println!("NotFound");
                    continue;
                };
                handler()
            }
        }
    });
}

#[bench]  // 10,738 ns/iter (+/- 143); 2022-11-16
fn double_hash_without_tupple(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..10 {
            let map = HashMap::from([
                (
                    "/",
                    HashMap::from([
                        (Method::GET,     sample_1 as fn()),
                        (Method::POST,    sample_2 as fn()),
                        (Method::CONNECT, sample_3 as fn()),
                    ])
                ),
                (
                    "/path1",
                    HashMap::from([
                        (Method::GET,     sample_4 as fn()),
                        (Method::POST,    sample_5 as fn()),
                        (Method::CONNECT, sample_6 as fn()),
                        (Method::DELETE,  sample_7 as fn()),
                    ])
                ),
                (
                    "/path2",
                    HashMap::from([
                        (Method::GET,     sample_4 as fn()),
                        (Method::POST,    sample_5 as fn()),
                        (Method::CONNECT, sample_6 as fn()),
                        (Method::DELETE,  sample_7 as fn()),
                        (Method::PATCH,   sample_8 as fn()),
                    ])
                ),
                (
                    "/path3",
                    HashMap::from([
                        (Method::POST,    sample_5 as fn()),
                        (Method::DELETE,  sample_7 as fn()),
                        (Method::GET,     sample_4 as fn()),
                        (Method::CONNECT, sample_3 as fn()),
                        (Method::PATCH,   sample_1 as fn()),
                    ])
                ),
            ]);
            for (method, path) in &CASES {
                let Some(handlers) = map.get(path) else {
                    println!("NotFound");
                    continue;
                };

                let Some(handler) = handlers.get(method) else {
                    println!("NotFound");
                    continue;
                };
                handler()
            }
        }
        
    })
}
