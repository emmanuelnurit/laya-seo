# Reserved sales and countdown display

A sale (commercial operation) can be public, as it always was, or reserved for
named customers. A reserved sale is invisible to anyone it is not open to: its
page answers 404, its discount is never served, and — when the merchant enables
`hide_products` — the products it covers disappear from listings, the front API,
the sitemap and the product page. A dated sale can also display a countdown on
the storefront: never, from N hours before the end date, or from its opening.

This document is the map for developers who work on the feature. The behavior
itself is specified by the test suites named below.
