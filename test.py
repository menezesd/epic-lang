import subprocess
import os
import sys

total = 0
passed = 0

for file in sorted(os.listdir('tests')):
    if not file.endswith('.epic'):
        continue
    input = 'tests/' + file
    output = 'tests/' + file[:-5] + '.out'
    total += 1
    
    print('running test ' + file + '...')
    jury_ans = open(output).read()
    try:
        got_ans = subprocess.check_output(['python3', 'interp.py', input], universal_newlines=True)
    except subprocess.SubprocessError as e:
        print('process finished badly: ' + str(e))

    if jury_ans == got_ans:
        print('test passed :)')
        passed += 1
    else:
        print('test failed :(')

print('{} out of {} tests passed'.format(passed, total))
if passed == total:
    print('good job ;)')
    sys.exit(0)
print('some tests failed :(')
sys.exit(1)