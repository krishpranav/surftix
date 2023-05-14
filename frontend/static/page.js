/**
 * @breif: DOM navigation stuffs
 */


// navigate forwards
function navigation_forward() {
    const url = new URL(window.location)
    const searchParams = url.searchParams

    let q = searchParams.get('q')
    let page = searchParams.get('page')

    if (page === null) {
        page = 2
        window.location = `${url.origin}${url.pathname}?q=${q}&page=${page}`
    } else {
        window.location = `${url.origin}${url.pathname}?q=${q}&page=${++page}`
    }
}

// navigate backwards
function navigation_backward() {
    const url = new URL(window.location)
    const searchParams = url.searchParams

    let q = searchParams.get('q')
    let page = searchParams.get('page')

    if (page !== null && page > 1) {
        window.location = `${url.origin}${url.pathname}?q=${q}&page=${--page}`
    }
}