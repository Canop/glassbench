// dom-doll by dystroy
// https://github.com/Canop/dom-doll

function $() {
	const parser = /^<([^ .#>]+)?(?:#([^ .#>]+))?(?:\.([^ #>]+))?(?:[^>]*>(.+))?$/
	var nodes
	for (let arg of arguments) {
		if (typeof arg == "string") {
			let parents = nodes || [document]
			nodes = []
			for (let parent of parents) {
				let creator = arg.match(parser)
				if (creator) {
					let e = document.createElement(creator[1] || "div")
					if (creator[2]) e.id = creator[2]
					if (creator[3]) e.className = creator[3].replaceAll('.', ' ')
					if (creator[4]) e.textContent = creator[4]
					if (parent != document) {
						parent.appendChild(e)
					}
					nodes.push(e)
				} else {
					for (let child of parent.querySelectorAll(arg)) {
						nodes.push(child)
					}
				}
			}
		} else if (typeof arg == "function") {
			if (nodes) nodes.forEach(arg)
			else nodes = arg()
		} else if (arg instanceof Element) {
			if (nodes) nodes[0].appendChild(arg)
			else nodes = [arg]
		} else if (Array.isArray(arg)) {
			for (let e of arg) {
				nodes[0].appendChild(e)
			}
		} else if (typeof arg == "object") {
			for (let e of nodes) {
				for (let attr in arg) {
					let val = arg[attr]
					if (typeof val == "function") {
						e.addEventListener(attr, val)
					} else if (["textContent", "innerHTML"].includes(attr)) {
						e[attr] = val
					} else {
						e.setAttribute(attr, val)
					}
				}
			}
		}
	}
	return (nodes && nodes.length==1) ? nodes[0] : nodes
}

const $$ = document.querySelectorAll.bind(document)


