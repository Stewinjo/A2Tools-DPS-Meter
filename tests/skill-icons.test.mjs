import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const source = readFileSync(new URL("../public/src/js/skillIcons.js", import.meta.url), "utf8");
const iconMap = readFileSync(new URL("../src/data/skill_icons.json", import.meta.url), "utf8");
const skill = { code: "18770000", job: "Chanter" };
const tick = () => new Promise((resolve) => setImmediate(resolve));

function setup(fetchSkillIcon) {
  const logs = [];
  const window = {
    javaBridge: {
      readResource: () => iconMap,
      logToDebug: (message) => logs.push(message),
      fetchSkillIcon,
    },
  };
  vm.runInNewContext(source, {
    window,
    console: { debug() {} },
    Blob,
    Uint8Array,
    atob,
    btoa,
    URL: { createObjectURL: () => "blob:icon" },
  });
  return { icons: window.skillIcons, logs };
}

function image() {
  return { dataset: {}, style: {}, classList: { remove() {} }, getAttribute(name) { return this[name]; } };
}

test("Chanter 1877 uses the game-data passive icon rather than guessing 077", () => {
  const { icons } = setup();
  assert.equal(icons.getIconCandidates(skill)[0], "https://assets.playnccdn.com/static-aion2-gamedata/resources/ICON_CH_SKILL_Passive_007.png");
});

for (const status of [404, 410]) {
  test(`${status} skips retries across concurrent images and redraws`, async () => {
    let requests = 0;
    const { icons, logs } = setup(async () => { requests += 1; return null; });
    const images = Array.from({ length: 4 }, image);
    images.forEach((img) => icons.applyIconToImage(img, skill));
    await tick();
    await tick();
    assert.equal(requests, 1);
    images.forEach((img) => assert.match(img.src, /^data:image\/svg/));
    const redrawn = image();
    icons.applyIconToImage(redrawn, skill);
    await tick();
    assert.match(redrawn.src, /^data:image\/svg/);
    assert.equal(requests, 1);
    assert.equal(logs.length, 1);
  });
}

test("a failed icon download never assigns the CDN url", async () => {
  let requests = 0;
  const { icons } = setup(async () => { requests += 1; throw new TypeError("Failed to fetch"); });
  const images = Array.from({ length: 4 }, image);
  images.forEach((img) => icons.applyIconToImage(img, skill));
  await tick();
  await tick();
  images.forEach((img) => {
    assert.match(img.src, /^data:image\/svg/);
    assert.equal(String(img.src).startsWith("https:"), false);
  });
  const redrawn = image();
  icons.applyIconToImage(redrawn, skill);
  await tick();
  assert.match(redrawn.src, /^data:image\/svg/);
  assert.equal(requests, 1);
});

test("successful downloads are shared and an old response cannot replace a new skill", async () => {
  let complete;
  let requests = 0;
  const { icons } = setup(() => { requests += 1; return new Promise((resolve) => { complete = resolve; }); });
  const first = image();
  const second = image();
  icons.applyIconToImage(first, skill);
  icons.applyIconToImage(second, skill);
  await tick();
  icons.applyIconToImage(first, { code: "" });
  const replacement = first.src;
  complete("iVBORw0KGgo=");
  await tick();
  await tick();
  assert.equal(first.src, replacement);
  assert.equal(second.src, "blob:icon");
  assert.equal(requests, 1);
  const third = image();
  icons.applyIconToImage(third, skill);
  assert.equal(third.src, "blob:icon");
});
