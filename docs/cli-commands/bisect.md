# openisl bisect

Binary-search the history to find the commit that introduced a bug.

## Synopsis

```bash
openisl bisect start <bad> <good>
openisl bisect good [revision]
openisl bisect bad [revision]
openisl bisect skip
openisl bisect reset
```

## Description

`openisl bisect start` begins a bisect session between a known-`bad` and known-`good` revision, then checks out the midpoint for you to test. Mark each tested revision with `good` or `bad`, and git narrows the search until the first bad commit is found. `skip` moves past a revision you can't test (e.g. it doesn't build). `reset` ends the session and returns to the original branch. Equivalent to `git bisect`.

## Commands

- `start <bad> <good>`: Begin a session
- `good [revision]`: Mark a revision as good (defaults to current)
- `bad [revision]`: Mark a revision as bad (defaults to current)
- `skip`: Skip the current revision
- `reset`: End the session

## Examples

```bash
openisl bisect start HEAD v1.0.0
openisl bisect good      # test passed on the current checkout
openisl bisect bad       # test failed
openisl bisect reset     # done
```

## See Also

- [openisl log](log.md) - Review history while bisecting
- [openisl checkout](checkout.md) - The revisions checked out during bisect