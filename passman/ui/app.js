let entries = [];
let selectedEntry = null;

const invoke = (...args) => window.__TAURI__.core.invoke(...args);

// --- Notification ---
let notifyTimer = null;
function notify(message, type = "success") {
  const el = document.getElementById("notification");
  el.textContent = message;
  el.className = "notification " + type;
  clearTimeout(notifyTimer);
  notifyTimer = setTimeout(() => {
    el.className = "notification hidden";
  }, 3000);
}

// --- Loading overlay ---
function showLoading() {
  document.getElementById("loading-overlay").classList.remove("hidden");
}
function hideLoading() {
  document.getElementById("loading-overlay").classList.add("hidden");
}

// --- Confirmation modal ---
function confirmModal(message) {
  return new Promise((resolve) => {
    const overlay = document.getElementById("modal-overlay");
    document.getElementById("modal-message").textContent = message;
    overlay.classList.remove("hidden");

    const onConfirm = () => { cleanup(); resolve(true); };
    const onCancel = () => { cleanup(); resolve(false); };

    function cleanup() {
      overlay.classList.add("hidden");
      document.getElementById("modal-confirm").removeEventListener("click", onConfirm);
      document.getElementById("modal-cancel").removeEventListener("click", onCancel);
    }

    document.getElementById("modal-confirm").addEventListener("click", onConfirm);
    document.getElementById("modal-cancel").addEventListener("click", onCancel);
  });
}

// --- Data ---
async function loadEntries() {
  try {
    entries = await invoke("get_entries");
    renderList();
  } catch (e) {
    notify("Failed to load entries: " + e, "error");
  }
}

function renderList() {
  const list = document.getElementById("entry-list");
  const query = (document.getElementById("search-input").value || "").toLowerCase();
  list.innerHTML = "";
  entries
    .filter((entry) => entry.name.toLowerCase().includes(query))
    .forEach((entry) => {
      const li = document.createElement("li");
      li.textContent = entry.name;
      if (selectedEntry && selectedEntry.name === entry.name) {
        li.classList.add("active");
      }
      li.addEventListener("click", () => selectEntry(entry));
      list.appendChild(li);
    });
}

function selectEntry(entry) {
  selectedEntry = entry;
  renderList();
  showEditForm(entry);
}

function createField(label, id, type, value) {
  const group = document.createElement("div");
  group.className = "form-group";

  const lbl = document.createElement("label");
  lbl.textContent = label;
  group.appendChild(lbl);

  const row = document.createElement("div");
  row.className = "input-row";

  const input = document.createElement("input");
  input.type = type;
  input.id = id;
  input.value = value || "";
  row.appendChild(input);

  if (type === "password") {
    const toggleBtn = document.createElement("button");
    toggleBtn.className = "btn-icon";
    toggleBtn.textContent = "Show";
    toggleBtn.addEventListener("click", () => {
      if (input.type === "password") {
        input.type = "text";
        toggleBtn.textContent = "Hide";
      } else {
        input.type = "password";
        toggleBtn.textContent = "Show";
      }
    });
    row.appendChild(toggleBtn);

    const copyBtn = document.createElement("button");
    copyBtn.className = "btn-icon btn-copy";
    copyBtn.textContent = "Copy";
    copyBtn.addEventListener("click", () => {
      navigator.clipboard.writeText(input.value).then(() => {
        copyBtn.textContent = "Copied!";
        notify("Password copied to clipboard");
        setTimeout(() => { copyBtn.textContent = "Copy"; }, 2000);
      });
    });
    row.appendChild(copyBtn);
  }

  group.appendChild(row);
  return group;
}

function closePanel() {
  selectedEntry = null;
  renderList();
  document.getElementById("main-panel").innerHTML =
    '<p class="placeholder">Select an entry or add a new one.</p>';
}

function showEditForm(entry) {
  const panel = document.getElementById("main-panel");
  panel.innerHTML = "";

  const header = document.createElement("div");
  header.className = "panel-header";
  const h3 = document.createElement("h3");
  h3.textContent = "Edit Entry";
  const closeBtn = document.createElement("button");
  closeBtn.className = "btn-close";
  closeBtn.title = "Close";
  closeBtn.innerHTML = '<svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="1" y1="1" x2="13" y2="13"/><line x1="13" y1="1" x2="1" y2="13"/></svg>';
  closeBtn.addEventListener("click", closePanel);
  header.appendChild(h3);
  header.appendChild(closeBtn);
  panel.appendChild(header);

  panel.appendChild(createField("Name", "f-name", "text", entry.name));
  panel.appendChild(createField("Username", "f-username", "text", entry.username));
  panel.appendChild(createField("Password", "f-password", "password", entry.password));
  panel.appendChild(createField("URL", "f-url", "text", entry.url));

  const notesGroup = document.createElement("div");
  notesGroup.className = "form-group";
  const notesLabel = document.createElement("label");
  notesLabel.textContent = "Notes";
  const notesArea = document.createElement("textarea");
  notesArea.id = "f-notes";
  notesArea.value = entry.notes || "";
  notesGroup.appendChild(notesLabel);
  notesGroup.appendChild(notesArea);
  panel.appendChild(notesGroup);

  const btnRow = document.createElement("div");
  btnRow.className = "btn-row";

  const saveBtn = document.createElement("button");
  saveBtn.className = "btn-save";
  saveBtn.textContent = "Save";
  saveBtn.addEventListener("click", () => saveEntry(entry.name));

  const delBtn = document.createElement("button");
  delBtn.className = "btn-delete";
  delBtn.textContent = "Delete";
  delBtn.addEventListener("click", () => deleteEntry(entry.name));

  btnRow.appendChild(saveBtn);
  btnRow.appendChild(delBtn);
  panel.appendChild(btnRow);
}

function showAddForm() {
  selectedEntry = null;
  renderList();
  const panel = document.getElementById("main-panel");
  panel.innerHTML = "";

  const header = document.createElement("div");
  header.className = "panel-header";
  const h3 = document.createElement("h3");
  h3.textContent = "Add Entry";
  const closeBtn = document.createElement("button");
  closeBtn.className = "btn-close";
  closeBtn.title = "Close";
  closeBtn.innerHTML = '<svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="1" y1="1" x2="13" y2="13"/><line x1="13" y1="1" x2="1" y2="13"/></svg>';
  closeBtn.addEventListener("click", closePanel);
  header.appendChild(h3);
  header.appendChild(closeBtn);
  panel.appendChild(header);

  panel.appendChild(createField("Name", "f-name", "text", ""));
  panel.appendChild(createField("Username", "f-username", "text", ""));
  panel.appendChild(createField("Password", "f-password", "password", ""));
  panel.appendChild(createField("URL", "f-url", "text", ""));

  const notesGroup = document.createElement("div");
  notesGroup.className = "form-group";
  const notesLabel = document.createElement("label");
  notesLabel.textContent = "Notes";
  const notesArea = document.createElement("textarea");
  notesArea.id = "f-notes";
  notesGroup.appendChild(notesLabel);
  notesGroup.appendChild(notesArea);
  panel.appendChild(notesGroup);

  const btnRow = document.createElement("div");
  btnRow.className = "btn-row";
  const addBtn = document.createElement("button");
  addBtn.className = "btn-save";
  addBtn.textContent = "Add";
  addBtn.addEventListener("click", addEntry);
  btnRow.appendChild(addBtn);
  panel.appendChild(btnRow);
}

async function addEntry() {
  const name = document.getElementById("f-name").value.trim();
  const username = document.getElementById("f-username").value.trim();
  const password = document.getElementById("f-password").value;
  const url = document.getElementById("f-url").value.trim();
  const notes = document.getElementById("f-notes").value.trim();

  if (!name) {
    notify("Name is required.", "error");
    return;
  }

  showLoading();
  try {
    await invoke("add_entry", { name, username, password, url, notes });
    await loadEntries();
    selectedEntry = entries.find((e) => e.name === name) || null;
    if (selectedEntry) selectEntry(selectedEntry);
    notify("Entry '" + name + "' added successfully");
  } catch (e) {
    notify(String(e), "error");
  } finally {
    hideLoading();
  }
}

async function saveEntry(oldName) {
  const name = document.getElementById("f-name").value.trim();
  const username = document.getElementById("f-username").value.trim();
  const password = document.getElementById("f-password").value;
  const url = document.getElementById("f-url").value.trim();
  const notes = document.getElementById("f-notes").value.trim();

  showLoading();
  try {
    await invoke("update_entry", { oldName, name, username, password, url, notes });
    await loadEntries();
    selectedEntry = entries.find((e) => e.name === name) || null;
    if (selectedEntry) selectEntry(selectedEntry);
    notify("Entry saved successfully");
  } catch (e) {
    notify(String(e), "error");
  } finally {
    hideLoading();
  }
}

async function deleteEntry(name) {
  const confirmed = await confirmModal("Are you sure you want to delete '" + name + "'?");
  if (!confirmed) return;

  showLoading();
  try {
    await invoke("delete_entry", { name });
    selectedEntry = null;
    await loadEntries();
    document.getElementById("main-panel").innerHTML =
      '<p class="placeholder">Select an entry or add a new one.</p>';
    notify("Entry '" + name + "' deleted");
  } catch (e) {
    notify(String(e), "error");
  } finally {
    hideLoading();
  }
}

async function refreshEntries() {
  const btn = document.getElementById("btn-refresh");
  btn.classList.add("spinning");
  try {
    await loadEntries();
    notify("Entries refreshed");
  } catch (e) {
    notify("Failed to refresh", "error");
  } finally {
    btn.classList.remove("spinning");
  }
}

function onTauriReady() {
  document.getElementById("btn-add").addEventListener("click", showAddForm);
  document.getElementById("btn-refresh").addEventListener("click", refreshEntries);
  document.getElementById("search-input").addEventListener("input", renderList);
  loadEntries();
}

if (window.__TAURI__) {
  onTauriReady();
} else {
  const check = setInterval(() => {
    if (window.__TAURI__) {
      clearInterval(check);
      onTauriReady();
    }
  }, 10);
}
