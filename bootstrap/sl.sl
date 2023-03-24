import "std/stdio.h"
import "std/stdlib.h"

enum JavaLikeEnum(x: int, y: int)
    Whatever(1, 2)
    Something(3, 4)
end

func main(argc: int, argv: const char**): int
    var x: JavaLikeEnum = JavaLikeEnum.Whatever

    printf("x.x = %d\n", x.x)
    printf("x.y = %d\n", x.y)

    return 0
end