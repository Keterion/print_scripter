# TODO
## 1
So I now got the function that can use all the numbers to stylize, now I've got to write something that can interpret the written commands (Like "[letter:100]" meaning a delay of 100ms between each word).
- [x] Interpret the written commands into commands for the printer -> basically done
- [x] Split the large function into multiple smaller ones to allow for more flexible commands -> won't do this because you couldn't have word and letter delay in one setting.
- [ ] maybe do that unscramble text thingy
- [x] maybe change the command thingy to a struct
- [x] implement the new regeX: (?:\[(\w+):(\d+)(?::(\d+))?){0,}(?:[{\]\s]([\w\s!.'_:,\?\\)(]+)[}\]\s]){0,} cuz it allows for this: [letter:50:3] with the second number being the ammount of words affected -> more modularity and flexibility (also is optional so no worries)
- [x] So now the stuff doesn't time anymore so I gotta do that