
import math

def size_in_bytes(number):
    import math
    if number == 0: return 1
    bits_needed = math.ceil(math.log2(abs(number) + 1))
    return math.ceil(bits_needed / 8)

print(size_in_bytes(1007812)) # 3