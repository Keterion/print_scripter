# TODO
## 1
So I now got the function that can use all the numbers to stylize, now I've got to write something that can interpret the written commands (Like "[letter:100]" meaning a delay of 100ms between each word).
- [x] Interpret the written commands into commands for the printer -> basically done
- [x] Split the large function into multiple smaller ones to allow for more flexible commands -> won't do this because you couldn't have word and letter delay in one setting.
- [ ] Add support for \n and stuff. -> scan for the \ for every character (split up the function first)

atm I check for one message, there can be a scan and also a message in one regex clump