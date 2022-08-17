import threading
import functools


def threadify(func):
    """Decorator to add threading to a function."""

    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        return threading.Thread(None, functools.partial(func, *args, **kwargs)).start()

    return wrapper


def gather(
    tasks,
    concurrency,
):
    """Run a list of tasks in parallel and return the results."""

    running = 0
    results = {}

    def _run_task(idx, func, args, kwargs):
        nonlocal running, results

        running += 1

        try:
            results[idx] = func(*args, **kwargs)
        except Exception as e:
            results[idx] = e

        running -= 1

    for idx, task in enumerate(tasks):
        while concurrency and (running >= concurrency):
            pass

        threadify(_run_task)(idx, *task)

    while running > 0:
        pass

    return dict(sorted(results.items()))
