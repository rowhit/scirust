
#![doc="A generics based scientific computing library for Rust


# Introduction

This library aims to provide scientific computing functionality in Rust.
Current focus is to provide a comprehensive API with simple straight-forward
implementations. Different modules cover functionality 
covering matrices, linear algebra, signal processing and statistics.



# Implementation

The implementation's aim to take the best advantage
of language features in Rust, yet avoid getting too much into
heavy degree of performance optimization. We do take 
advantage of Rust features like generic programming, 
immutable by default approach, type traits, iterators,
amongst others. This is a pure Rust implementation. 
There is no integration planned at the moment with
C libraries like BLAS or LAPACK.  


"]

#![feature(unsafe_destructor, globs)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;



pub mod discrete{
    #![doc="Discrete mathematics
    "]

    pub use self::modular::*;

    mod modular;

}

pub mod matrix {
    #![doc="Fundamental matrix structures
    "]

    pub use self::element::*;
    pub use self::error::*;
    pub use self::iter::*;
    pub use self::matrix::*;
    pub use self::random::*;
    pub use self::constructors::*;
    pub use self::view::*;
    pub use self::traits::*;

    mod element;
    mod error;
    mod iter;
    mod matrix;
    mod random;
    mod constructors;
    mod view;
    mod traits;
    // for internal use only
    pub mod testdata;
}


pub mod linalg {
    #![doc="Linear algebra algorithms
    "]
    pub use self::gauss_elim::*;
    pub use self::det::*;
    pub use self::error::*;

    mod gauss_elim;
    mod det;
    mod error;
}


pub mod stat {
    #![doc="Statistics
    "]
}

pub mod opt {
    #![doc="Optimization
    "]
    pub mod lp {
    #![doc="Linear programming
    "]
    }
    pub mod ls {
    #![doc="Least squares
    "]
    }
    pub mod cvx {
    #![doc="Convex optimization
    "]
    }
}



pub mod signal {
    #![doc="Signal processing
    "]    
}


pub mod image {
    #![doc="Image processing
    "]
}

pub mod audio {
    #![doc="Audio signal processing
    "]
}
