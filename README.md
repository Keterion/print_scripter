# Syntax
## Grouping
You bind settings to specified groups like so:
`[setting:time][setting:time]{Text}`
## In-Text commands
You can use all of the commands in curly braces.


Example:


`"{Hello [letter:\100:1]World!}"`
## Formatting
\ escape characters don't work anymore. Now the curly braces are used to set a linebreak.

# The use thingies
time is in the format of milliseconds
|   Command  |Parameter|    Parameter   |                          Function                           |
|:----------:|:-------:|:--------------:|:-----------------------------------------------------------:|
| letter / l |   time  | words affected |Sets the delay between the single letters|
| word / w   |   time  | words affected |Sets the delay between words, works together with the letters|
| wait       |   time  |                |Pauses for the specified ammount of time|
| {}         |         |                |You put the text in between these|


For an example type !test! into the console when launching
