import atexit
import os
import readline
from pathlib import Path

# History file
history_file = Path(os.environ["LOCALDIR"]) / ".python_history"
readline.parse_and_bind("tab: complete")
try:
    readline.read_history_file(history_file)
except IOError:
    pass
atexit.register(readline.write_history_file, history_file)
