# Rust Vertical Datum Converter

**Created by**: _Ryan Gilchrist_

### Overview

This tool is my first attempt at writing something useful in the Rust programming language. It converts vertical datums between the following:

- Normal Amsterdam Level (NAL)
- Lowest Astronomical Tide (LAT)

### Constraints

- The latitude and longitude must be within the bounds of the input data
- The latest data update is from 2018, available from the [Ministry of Defence](https://english.defensie.nl/topics/hydrography/documents/applications/2020/06/12/nllat2018)

### Proof of Correct Function

I haven't written any tests for this program yet, because I haven't yet learned how to in Rust! My plans are to eventually rewrite the program when I'm more experienced with using Rust.

Therefore, please manually check the output of this program is what you are expecting, if the value is being used for an important purpose.
