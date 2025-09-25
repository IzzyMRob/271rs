#[macro_export]
macro_rules! max {
  ( $x:expr, $y:expr ) => {
      if $x > $y { $x } else { $y }
  };
}

#[macro_export]
macro_rules! min {
    ( $x:expr, $y:expr ) => {
        if $x < $y {$x} else {$y}
    };
}

// if e then f else g
#[macro_export]
macro_rules! choice {
    ( $e:expr, $f:expr, $g:expr ) => {
        ($e & $f) | (!$e & $g);
    };
}

//if x ==y x else if x == z x else z
#[macro_export]
macro_rules! median {
    ( $x:expr, $y:expr, $z:expr ) => {
        $x & $y | $x & $z | $y & $z
    };
}

// move left by y, then move right by length-y and use that to fill in black spaces
// return the new value
#[macro_export]
macro_rules! rotatel {
    ( $x:expr, $y:expr ) => {
        ($x << $y) | ($x >> ((size_of_val(&$x)*8) - $y%))
    };
}

// opposite of above
#[macro_export]
macro_rules! rotate {
    ( $x:expr, $y:expr ) => {
        ($x >> $y) | ($x << ((size_of_val(&$x)*8) - $y))
    };
}
