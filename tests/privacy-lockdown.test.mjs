import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const read = (path) => readFileSync(new URL(`../${path}`, import.meta.url), "utf8");

const conf = JSON.parse(read("src-tauri/tauri.conf.json"));
const hosts = (() => {
  const rust = read("src-tauri/src/open_urls.rs");
  const start = rust.indexOf("BROWSER_HOSTS");
  const end = rust.indexOf("];", start);
  return [...rust.slice(start, end).matchAll(/"([^"]+)"/g)].map((match) => match[1]);
})();

const hostAllowed = (host) => {
  const name = host.toLowerCase().replace(/\.$/, "");
  return hosts.some((allowed) => name === allowed || name.endsWith(`.${allowed}`));
};

test("the webview CSP has no external image or connect host", () => {
  for (const key of ["csp", "devCsp"]) {
    const policy = conf.app.security[key];
    assert.equal(policy["img-src"].includes("http"), false, key);
    const connect = policy["connect-src"];
    assert.equal(connect.includes("https:"), false, key);
    assert.equal(connect.includes("playnccdn"), false, key);
  }
  assert.match(conf.app.security.csp["connect-src"], /http:\/\/ipc\.localhost/);
  assert.match(conf.app.security.devCsp["connect-src"], /ws:\/\/localhost:1420/);
  assert.match(conf.app.security.csp["script-src"], /'self'/);
});

test("pages that open links only name allowlisted https hosts", () => {
  assert.ok(hosts.includes("github.com"));
  assert.ok(hosts.includes("discord.gg"));
  assert.ok(hosts.includes("ko-fi.com"));
  const pages = ["index.html", "public/src/js/core.js", "public/src/js/tauriBridge.js"];
  for (const page of pages) {
    const urls = read(page).match(/https:\/\/[^\s"'<>)]+/g) || [];
    assert.ok(urls.length > 0, page);
    for (const raw of urls) {
      const host = new URL(raw).hostname;
      assert.equal(hostAllowed(host), true, `${page} opens ${raw}`);
    }
  }
});

test("skill icons are not fetched by the page", () => {
  const icons = read("public/src/js/skillIcons.js");
  const details = read("public/src/js/details.js");
  assert.equal(icons.includes("fetch("), false);
  assert.equal(details.includes("fetch("), false);
  assert.match(icons, /https:\/\/assets\.playnccdn\.com\/static-aion2-gamedata\/resources/);
  assert.match(icons, /fetchSkillIcon/);
});

test("the default build is the private one", () => {
  const cargo = read("src-tauri/Cargo.toml");
  assert.match(cargo, /default = \["desktop"\]/);
  const pkg = JSON.parse(read("package.json"));
  assert.match(pkg.scripts["build:offline"], /--features desktop/);
  assert.doesNotMatch(pkg.scripts["build:offline"], /online/);
  assert.match(pkg.scripts["build:online"], /--features online/);
});
