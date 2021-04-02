
async function main(sql_conf){
	let db_bytes = base64ToArrayBuffer(db64)
	const SQL = await initSqlJs(sql_conf)
	const db = new SQL.Database(new Uint8Array(db_bytes))
	let benches = db.exec("SELECT * FROM bench")
	window.gb = wrap(db)
	create_gui()
	make_selector(gb_conf.bench_name, gb_conf.task_name)
}

function create_gui() {
	$("body",
		$("<#infos",
			$(`<a>Glassbench ${gb_conf.gb_version}`, {
				href: "https://github.com/Canop/glassbench",
				target: "_blank",
			}),
		),
		$("<#selectors"),
		$("<#view",
			$("<.tabs"),
			$("<.pages"),
		)
	)
	function unselect() {
		$("#view .tabs .tab, #view .pages .page", e => {
			e.classList.remove("selected")
		})
	}
	;["Table", "Graph"].forEach(name => {
		unselect()
		let page = $("<.page.selected", { id: name })
		let tab = $("<span.tab.selected", {
			textContent: name,
			click: () => {
				unselect()
				tab.classList.add("selected")
				page.classList.add("selected")
			}
		})
		$("#view .tabs", tab)
		$("#view .pages", page)
	})
	$("#Table",
		$("<.table-wrapper",
			$("<table",
				$("<thead", $("<tr",
					$("<th.group>dataset"),
					$("<th.bench_id>bench id"),
					$("<th.commit_id>commit id"),
					$("<th.date>date"),
					$("<th.task_name>task name"),
					$("<th.duration_str>mean dur."),
					$("<th.duration_ns>mean (ns)"),
					$("<th.tag>tag"),
				)),
				$("<tbody#tbody")
			)
		)
	)
	$("#Graph", $("<#vis"))
}

function update_view() {
	let view_data = make_view_data()
	update_graph(view_data)
	update_table(view_data)
}

// return [{
// 	group_id, group_name, bench_name, task_name, include_tag, tag,
// 	rows:[{date,duration_ns,tag,duration_str}]
// }]
function make_view_data() {
	let groups = []
	for (let [group_id, selector] of $$(".selector").entries()) {
		let bench_name = $(selector, "select.bench").value
		let task_name = $(selector, "select.task").value
		let include_tag = !$(selector, "select.tag-toggle").selectedIndex
		let tag = $(selector, ".tag").value
		let rows = get_rows(bench_name, task_name, include_tag, tag)
		let group_name = task_name
		groups.push({
			group_id,
			group_name,
			bench_name,
			include_tag,
			tag,
			task_name,
			rows,
		})
	}
	return groups
}

// return [{date,tag,duration_ns,duration_str}]
function get_rows(bench_name, task_name, include_tag, tag) {
	let sql = `SELECT
			bench.id, bench.time, bench.tag, bench.commit_id,
			task.iterations, task.mean_duration_ns
		FROM task JOIN bench ON task.bench=bench.id
		WHERE bench.name=? AND task.name=?`
	let args = [bench_name, task_name]
	if (tag) {
		if (include_tag) {
			sql += " AND tag LIKE ?"
		} else {
			sql += " AND (tag IS NULL OR tag NOT LIKE ?)"
		}
		args.push(`%${tag}%`)
	}
	return gb.lists(sql, args).map(row => ({
		bench_id: row[0],
		commit_id: row[3],
		date: row[1] * 1000,
		duration_ns: row[5],
		tag: row[2],
		duration_str: fmt_nanos(row[5])
	}))
}

function update_table(view_data) {
	let tbody = $("#tbody")
	while (tbody.firstChild) tbody.removeChild(tbody.lastChild)
	for (let g of view_data) {
		for (let row of g.rows) {
			$(tbody, $(`<tr.group_${g.group_id}`,
				$(`<td.group_id>${g.group_id + 1}`), // counting from 1
				$("<td.bench_id", { textContent: row.bench_id }),
				$("<td.commit_id", { textContent: row.commit_id.slice(0, 10) }),
				$("<td.date", { textContent: new Date(row.date) }),
				$("<td.task_name", { textContent: g.task_name }),
				$("<td.duration_str", { textContent: row.duration_str }),
				$("<td.duration_ns", { textContent: row.duration_ns }),
				$("<td.tag", { textContent: row.tag }),
			))
		}
	}
}

function update_graph(view_data) {
	if (window.graph) window.graph.destroy()
	let groups = new vis.DataSet()
	let items = []
	let min_date
	let max_date
	for (let g of view_data) {
		let group = {
			id: g.group_id,
			content: g.group_name,
		}
		for (let row of g.rows) {
			if (!(min_date<row.date)) min_date = row.date
			if (!(max_date>row.date)) max_date = row.date
			items.push({
				x: row.date,
				y: row.duration_ns,
				group: g.group_id,
				label: { content: row.duration_str }
			})
		}
	}
	var options = {
		start: min_date - (max_date-min_date)/10,
		end: max_date + (max_date-min_date)/10,
		shaded: true,
	}
	window.graph = new vis.Graph2d($("#vis"), items, groups, options)
	graph.on('click', function (properties) {
		console.log("click", properties)
	})
}

function make_selector(bench_name, task_name) {
	let bench_name_select = $("<select.bench",
		{ change: update_task_name_select },
		gb.bench_names.map(bn => $(`<option>${bn}`))
	)
	if (bench_name) {
		let idx = gb.bench_names.indexOf(bench_name)
		if (idx >= 0) bench_name_select.selectedIndex = idx
	}
	let task_name_select = $("<select.task", { change: update_view })
	function update_task_name_select(){
		let bench_name = bench_name_select.value
		console.log('bench_name:', bench_name);
		let bench_id = gb.val("SELECT id FROM bench WHERE name=$name", { $name: bench_name })
		let task_names = gb
			.list(
				"SELECT name FROM task where bench=$bench_id",
				{ $bench_id: bench_id }
			)
		task_name_select.innerHTML = ""
		$(task_name_select, task_names.map(name => $(`<option>${name}`)))
		return task_names
	}
	let tag_input = $("<input.tag", {
		change: update_view,
		keyup: update_view,
	})
	let task_names = update_task_name_select()
	if (task_name) {
		let idx = task_names.indexOf(task_name)
		if (idx >= 0) task_name_select.selectedIndex = idx
	}
	let wrapper = $("<.selector-wrapper",
		$("<.selector",
			$("<label>bench:", bench_name_select),
			$("<label>task:", task_name_select),
			$("<select.tag-toggle", { change: update_view },
				$("<option>with tag"),
				$("<option>without tag"),
			),
			tag_input,
			$("<.legend-icon"),
			$("<button.remover>-", {
				click: () => {
					wrapper.remove()
					update_view()
				}
			})
		),
		$("<.adder",
			$("<button>+", { click: make_selector })
		)
	)
	$("#selectors", wrapper)
	update_view()
}

function wrap(db) {
	let gb = {}
	gb.val = (sql, args) => gb.list(sql, args)[0]
	gb.list = (sql, args) => {
		let res = db.exec(sql, args)[0]
		return res ? res.values.map(r => r[0]) : []
	}
	gb.lists = (sql, args) => {
		let res = db.exec(sql, args)[0]
		return res ? res.values : []
	}
	gb.bench_names = gb.list("SELECT DISTINCT(name) FROM bench")
	return gb
}

function base64ToArrayBuffer(base64) {
	var binary_string = window.atob(base64)
	var len = binary_string.length
	var bytes = new Uint8Array(len)
	for (var i = 0; i < len; i++) {
		bytes[i] = binary_string.charCodeAt(i)
	}
	return bytes.buffer
}

function fmt_nanos(nanos) {
	if (nanos < 1000) {
		return nanos + "ns"
	}
	const num = n => n > 999 ? Math.round(n) : n.toFixed(2)
	let micros = nanos / 1000
	if (micros < 1000) {
		return num(micros) + "µs"
	}
	let millis = micros / 1000
	if (millis < 1000) {
		return num(millis) + "ms"
	}
	let seconds = millis / 1000
	return num(seconds) + "s"
}
