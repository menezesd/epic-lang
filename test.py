total = 0
import subprocess
import os
import sys
from typing import NoReturn

def main() -> NoReturn:
    """Run all .epic tests in the tests directory and compare output."""
    total: int = 0
    passed: int = 0
    for file in sorted(os.listdir('tests')):
        if not file.endswith('.epic'):
            continue
        input_path: str = os.path.join('tests', file)
        output_path: str = os.path.join('tests', file[:-5] + '.out')
        total += 1
        print(f'running test {file}...')
        with open(output_path) as f:
            jury_ans: str = f.read().rstrip()
        try:
            got_ans: str = subprocess.check_output(['python3', 'interp.py', input_path], universal_newlines=True).rstrip()
        except subprocess.CalledProcessError as e:
            print('process finished badly: ' + str(e))
            got_ans = ''
        if jury_ans == got_ans:
            print('test passed :)')
            passed += 1
        else:
            print('test failed :(')
            print('Expected:')
            print(jury_ans)
            print('Got:')
            print(got_ans)
    print(f'{passed} out of {total} tests passed')
    if passed == total:
        print('good job ;)')
        sys.exit(0)
    print('some tests failed :(')
    sys.exit(1)

if __name__ == '__main__':
    main()