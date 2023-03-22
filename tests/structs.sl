import "std/stdio.h"
import "std/stdlib.h"

struct Person
	name: const char*
	age: int

	constructor: func(const char*, int): Person*
	say_hi: func(Person*)
end

func Person.say_hi(self: Person*)
	printf("Hi, my name is %s and I'm %d years old\n", self.name, self.age)
end

func main(argc: int, argv: const cstring*)
	var john: Person* = new Person("John", 20)
	john.say_hi(john)
end

