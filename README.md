# Adblock-coffee

This project is a simplest Java wrapper for the `adblock-rust` library, allowing you to use the powerful ad-blocking capabilities of `adblock-rust` in your Java applications.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)

## Introduction

`adblock-rust` is a high-performance ad-blocking library written in Rust. This project provides a Java wrapper around `adblock-rust`, enabling Java developers to integrate ad-blocking functionality into their applications seamlessly.

## Features

- High-performance ad-blocking using `adblock-rust`.
- Easy-to-use Java API.
- Cross-platform support.

## Installation

### Prerequisites

- Java Development Kit (JDK) 8 or higher.
- Rust and Cargo (for building the native library).

### Building the Native Library

First, you need to build the `adblock-rust` library and generate the shared library file (`.so`, `.dll`, or `.dylib` depending on your OS).

1. Clone the `adblock-rust` repository:
2. Build the library: 
```sh 
cargo build --release --manifest-path adblock-rs/Cargo.toml
mvn package
```

3. Locate the generated shared library file in the `target/release` directory.
```java
public class Main {    
    public static void main(String[] args) {
        List<String> rules = new ArrayList<>(List.of(
            "-advertisement-icon.",
            "-advertisement-management/",
            "-advertisement.",
            "-advertisement/script."
        ));
        
        AdvtBlocker blocker = AdvtBlocker.createInstance(rules);
    }
}
```

### Integrating with Your Java Project

1. Copy the shared library file to a directory accessible by your Java application.

2. Add the Java wrapper library to your project. You can do this by including the JAR file or adding the source code directly to your project.

## Usage

### Loading the Native Library

Before using the wrapper, you need to load the native library. This can be done using `System.loadLibrary` or `System.load`.

### Using the Wrapper

Here is an example of how to use the Java wrapper to block ads:

```java
import com.example.adblock.AdblockEngine;

public class Main {
    public static void main(String[] args) {
        List<String> rules = new ArrayList<>(List.of(
            "-advertisement-icon.",
            "-advertisement-management/",
            "-advertisement.",
            "-advertisement/script."
        ));
        
        AdvtBlocker blocker = AdvtBlocker.createInstance(rules);
        boolean result = blocker.checkUrls(
            "http://example.com/-advertisement-icon.",
            "http://example.com/helloworld",
            "image"
        );
        
        System.out.println(result);
    }
}
```
