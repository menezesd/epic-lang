import subprocess
import os
import sys
from typing import NoReturn

def main() -> NoReturn:
    """Run all .epic tests in the tests directory and compare output."""
    # Check for mode flags
    use_analyze = '--analyze' in sys.argv

    if use_analyze:
        mode_name = "analyzing"
        mode_flag = '--analyze'
    else:
        mode_name = "tree-walking"
        mode_flag = None

    print(f"Running tests in {mode_name} mode...")

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
            cmd = ['python3', 'interp.py']
            if mode_flag:
                cmd.append(mode_flag)
            cmd.append(input_path)
            got_ans: str = subprocess.check_output(
                cmd,
                universal_newlines=True,
                stderr=subprocess.STDOUT
            ).rstrip()
        except subprocess.CalledProcessError as e:
            got_ans = (e.output or '').rstrip()
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