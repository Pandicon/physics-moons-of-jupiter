This is a part of my physics homework. One part of it was finding a sine wave going through four points, which were not measured exactly though.

## Sine wave finder
This part actually finds the best wave. You input the data into it in the following format: First three lines include the starting value, minimum value, maximum value, and amount of steps it should take to go over the interval for each of the three parameters of the wave: The amplitude, the shift, and the frequency.
Then you can input as many points as you wish in the format `x y dy`, where dy is the error of the measurement. Once you are done inputting the points, put an `s` there.

The program will output every wave found to be the currently best one, so the last one outputted is the overall best.

### Running
Run it with `sine-wave-finder.exe < input_file`, so for example `sine-wave-finder.exe < input_callisto.txt`. If you are not on Windows, you will have to build it yourself using [Rust](https://www.rust-lang.org/), for example via `cargo build --release`.

To generate raw output, which can be used as input for the sine wave visualiser, you can run it with the `--raw-output` flag, so for example `sine-wave-finder.exe --raw-output < input_callisto.txt > output.txt`. I recommend redirecting the output to a text file to be able to use it for the visualiser easily.

### Output
The output is each new best wave, where A is the amplitude, B is the shift in radians, F is the frequency, and cost is the "badness" of the wave. If you generate raw output, the numbers won't be marked, but they will be outputted in the format `A B F cost`.

## Sine wave visualiser
Basically use the raw output of the wave finder for this as I don't see much point in creating your own input. But if you wanted to, it has to include the points in the same format as for the wave finder, then a `---` separator line, and then each line has to look like `a b f c`, where `a` is the amplitude, `b` is the shift (in radians), `f` is the frequency, and `c` is the cost (aka "badness") of the wave.
If you want to animate more waves, just throw in a `---` separator between the lines for each wave. You will have to include the points for each wave separately. Look at the [`my_waves.txt`](https://github.com/Pandicon/physics-moons-of-jupiter/blob/main/sine-wave-visualiser/my_waves.txt) file for an example.

The program will then iterate over the waves and display them, showing the progress of the guess getting better and better with time (if you use the output of the wave finder).

### Running
Run it with `sine-wave-visualiser.exe < input_file`, so for example `sine-wave-visualiser.exe < my_waves.txt`. Once again if you are not on Windows, you will have to build it yourself using [Rust](https://www.rust-lang.org/), for example via `cargo build --release`.

## Pre-built binaries
I currently only offer Windows binaries. You can find them in the [releases](https://github.com/Pandicon/physics-moons-of-jupiter/releases/latest) section.
