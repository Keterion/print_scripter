# Syntax
## Grouping
You bind settings to specified groups like so:
[setting:time;setting:time]{Text}
[wait:time] -> stops the printing for time in ms
## In-Text commands
You can use all of the commands in curly braces:
"{Hello [letter:100]World!}"
## Formatting
\ escape characters don't work anymore. Now the curly braces are used to set a line.

# The use thingies
time is in the format of milliseconds
|   Command  |Parameter|Function|
|------------|---------|--------|
| letter / l |   time  |sets the delay between the single letters|
| word / w   |   time  |sets the delay between words, works together with the letters|
| wait       |   time  |pauses for the specified ammount of time|
| {}         |         |You put the text in between these|
