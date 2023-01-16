# Syntax
## Grouping
You bind settings to specified groups like so:
[setting:time][setting:time]{Text}
## In-Text commands
You can use all of the commands in curly braces.


Example:


"{Hello [letter:100]World!}"
## Formatting
\ escape characters don't work anymore. Now the curly braces are used to set a line.

# The use thingies
time is in the format of milliseconds
|   Command  |Parameter|    Parameter   |                          Function                           |
|:----------:|:-------:|:--------------:|:-----------------------------------------------------------:|
| letter / l |   time  | words affected |Sets the delay between the single letters|
| word / w   |   time  | words affected |Sets the delay between words, works together with the letters|
| wait       |   time  | words affected |Pauses for the specified ammount of time|
| {}         |         | words affected |You put the text in between these|
