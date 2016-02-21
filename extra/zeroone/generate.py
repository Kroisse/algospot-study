import random

length = 1000000
n = 100000

with open('input2.txt', 'w') as f:
    for _ in xrange(length // 2):
        f.write('01')
    f.write('\n')
    f.write('{}\n'.format(n))
    for _ in xrange(n):
        f.write('{} {}\n'.format(random.randrange(length), random.randrange(length)))
