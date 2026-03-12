# Project Notes for the OpenBSD Port of Codex

Building a new package from a new upstream release can take hours when
performing all the tests. When updating the port for new upstream
releases, track the time spent in each step so that timing estimates
for new releases can be identified and used to inform future
performance improvements. Linking operations in particular seem to
take a long time and should be measured. Long wall-clock is acceptable
if it is justified.

Create patches only when changes are needed specifically to account
for OpenBSD Porting. Do not patch tests. Exclude the tests instead to
be patched upstream.
