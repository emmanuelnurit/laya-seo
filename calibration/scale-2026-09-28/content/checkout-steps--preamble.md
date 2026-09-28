# Configurable checkout steps and display form

The checkout used to be written into the theme: three pages plus a
confirmation, each page checking the cart by hand and redirecting on its own.
It is now described by configuration: an ordered list of steps stored in the
database, each step bound to the guard it demands, and a shop setting that
picks the display form — one page per step, as before, or every step stacked
on a single screen.

A merchant can turn off a step the shop does not use (a download-only shop
drops the delivery step). Turning a step off removes its screen, never its
check: `CheckoutValidationService::validateForOrder()` runs the `check()` of
every registered step provider when the order is placed — the four of the core
and any a module ships — in declared-position order, and reads neither the
`active` flag nor `isSkippedFor()` to decide which. The cart opens the tunnel,
the payment comes next to last and the confirmation closes it; the back office
refuses any other shape, and a configuration broken behind its back falls back
to the defaults with a log warning instead of refusing to render.

Deploy the SQL before the code: `checkout_step` is read on every page of the
tunnel, and a missing table falls back to the providers' defaults with a log
warning rather than 500-ing the whole checkout.

This document is the map for developers who work on the feature. The behavior
itself is specified by the test suites named below.
