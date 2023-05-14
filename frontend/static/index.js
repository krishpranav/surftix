/**
 * @brief: index file[query and some other things]
 * 
 */

let search_box = document.querySelector('input') // search box

// search via web
function search_web() {
  window.location = `search?q=${search_box.value}`
}

// event listener
search_box.addEventListener('keyup', (e) => {
  if (e.keyCode === 13) {
    search_web()
  }
})