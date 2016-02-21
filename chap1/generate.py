import random

cases = 100

with open('input2.txt', 'w') as f:
    f.write('{}\n'.format(cases))
    for _ in xrange(cases):
        n = random.randint(1, 1000)
        k = random.randint(1, n)
        f.write('{} {}\n'.format(n, k))
        f.write(' '.join(str(random.randint(1, 100)) for _ in xrange(n)))
        f.write('\n')
