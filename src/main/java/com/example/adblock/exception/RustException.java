package com.example.adblock.exception;

import java.lang.RuntimeException;

/**
 * <code>RustException</code> extended base RuntimeException to catching exceptions form external rust-library.
 */
public class RustException extends RuntimeException {

    /**
     * Default constructor. Message indicates about runtime exception.
     */
    public RustException() {
        super("Rust runtime error");
    }

    /**
     * Construct a new RustException with the given message.
     *
     * @param msg The exception message.
     */
    public RustException(String message) {
        super(message);
    }

    /**
     * Construct a new RustException with the given message and cause
     *
     * @param msg   The exception message.
     * @param cause The exception cause.
     */
    public RustException(String msg, Throwable cause) {
        super(msg, cause);
    }
}

