# Look Up Table
Look up tables mimic the evaluation of functions at different points. If a function is complicated
or time taking to evaluate at any given point, one can use a Look Up Table, which contains the
pre-evaluated sample points of the function.
Later when an evaluation is requested, if the sample is directly found, the corresponding value is
returned. On the other hand, if the value is not found, then an interpolation (most of the times
it is linear to get better performance) of the values is performed to get an estimate of the
actual function. In practice this gives a reasonable approximation of the function.
Ofcourse when the values are out of bounds, then the last values are returned always.

This library currently supports linear interpolation in 1-D and 2-D only. Plan is to expand the interpolation and extrapolation options, along with generalizing the code to N-dimensions.
