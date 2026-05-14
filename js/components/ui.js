import React from "react";
import { Link } from "react-router-dom";
import * as Sentry from "@sentry/browser";

import * as wasm from "../../crate/pkg/sandtable_bg.wasm";
import { Species } from "../../crate/pkg/sandtable";
const memory = wasm.memory;

import { height, universe, width, reset, safePushUndo } from "../index.js";
import { snapshot, pallette } from "../render.js";
import { functions, storage } from "../api.js";
import SignInButton from "./signinButton.js";
import Promotab from "./promotab";
import { svgToImageData, rgbaToSpecies } from "../convertSVG";

import Menu from "./menu";

window.species = Species;
let pallette_data = pallette();

const UI_TEXT = {
  en: {
    browse: "Browse",
    settings: "Settings",
    reset: "Reset",
    info: "Info",
    upload: "Upload",
    wind: "Wind",
    naturalDisasters: "Natural Disasters",
    calm: "Calm",
    rain: "Rain",
    typhoon: "Typhoon",
    earthquake: "Earthquake",
    lightning: "Lightning Storm",
    meteor: "Meteor Shower",
    powders: "Powders",
    liquids: "Liquids",
    gases: "Gases",
    structures: "Structures",
    chaos: "Chaos",
    settingsTitle: "Settings",
    maxFps: "Max FPS",
    language: "Language",
    close: "Close",
    unlimited: "Unlimited",
    langEn: "English",
    langKo: "Korean",
    langJa: "Japanese",
    langZh: "Chinese",
    langDe: "German",
  },
  ko: {
    browse: "탐색",
    settings: "설정",
    reset: "초기화",
    info: "정보",
    upload: "업로드",
    wind: "바람",
    naturalDisasters: "자연재해",
    calm: "고요",
    rain: "비",
    typhoon: "태풍",
    earthquake: "지진",
    lightning: "뇌우",
    meteor: "유성우",
    powders: "가루",
    liquids: "액체",
    gases: "기체",
    structures: "구조물",
    chaos: "혼돈",
    settingsTitle: "설정",
    maxFps: "최대 FPS",
    language: "언어",
    close: "닫기",
    unlimited: "제한 없음",
    langEn: "영어",
    langKo: "한국어",
    langJa: "일본어",
    langZh: "중국어",
    langDe: "독일어",
  },
  ja: {
    browse: "閲覧",
    settings: "設定",
    reset: "リセット",
    info: "情報",
    upload: "アップロード",
    wind: "風",
    naturalDisasters: "自然災害",
    calm: "穏やか",
    rain: "雨",
    typhoon: "台風",
    earthquake: "地震",
    lightning: "雷雨",
    meteor: "流星群",
    powders: "粉体",
    liquids: "液体",
    gases: "気体",
    structures: "構造物",
    chaos: "カオス",
    settingsTitle: "設定",
    maxFps: "最大FPS",
    language: "言語",
    close: "閉じる",
    unlimited: "無制限",
    langEn: "英語",
    langKo: "韓国語",
    langJa: "日本語",
    langZh: "中国語",
    langDe: "ドイツ語",
  },
  zh: {
    browse: "浏览",
    settings: "设置",
    reset: "重置",
    info: "信息",
    upload: "上传",
    wind: "风",
    naturalDisasters: "自然灾害",
    calm: "平静",
    rain: "下雨",
    typhoon: "台风",
    earthquake: "地震",
    lightning: "雷暴",
    meteor: "流星雨",
    powders: "粉体",
    liquids: "液体",
    gases: "气体",
    structures: "结构",
    chaos: "混沌",
    settingsTitle: "设置",
    maxFps: "最大 FPS",
    language: "语言",
    close: "关闭",
    unlimited: "无限制",
    langEn: "英语",
    langKo: "韩语",
    langJa: "日语",
    langZh: "中文",
    langDe: "德语",
  },
  de: {
    browse: "Durchsuchen",
    settings: "Einstellungen",
    reset: "Zurücksetzen",
    info: "Info",
    upload: "Hochladen",
    wind: "Wind",
    naturalDisasters: "Naturkatastrophen",
    calm: "Ruhig",
    rain: "Regen",
    typhoon: "Taifun",
    earthquake: "Erdbeben",
    lightning: "Gewitter",
    meteor: "Meteorregen",
    powders: "Pulver",
    liquids: "Flüssigkeiten",
    gases: "Gase",
    structures: "Strukturen",
    chaos: "Chaos",
    settingsTitle: "Einstellungen",
    maxFps: "Max FPS",
    language: "Sprache",
    close: "Schließen",
    unlimited: "Unbegrenzt",
    langEn: "Englisch",
    langKo: "Koreanisch",
    langJa: "Japanisch",
    langZh: "Chinesisch",
    langDe: "Deutsch",
  },
};

const ELEMENT_LABELS = {
  en: {
    Sand: "Sand",
    Dust: "Dust",
    Stone: "Stone",
    Glass: "Glass",
    Water: "Water",
    Oil: "Oil",
    Acid: "Acid",
    Lava: "Lava",
    Slime: "Slime",
    Nitro: "Nitro",
    Gas: "Gas",
    Steam: "Steam",
    Fire: "Fire",
    Wall: "Wall",
    Wood: "Wood",
    Plant: "Plant",
    Fungus: "Fungus",
    Seed: "Seed",
    Metal: "Metal",
    Electricity: "Electricity",
    Rocket: "Rocket",
    Cloner: "Cloner",
    Mite: "Mite",
    Ice: "Ice",
  },
  ko: {
    Sand: "모래",
    Dust: "먼지",
    Stone: "돌",
    Glass: "유리",
    Water: "물",
    Oil: "기름",
    Acid: "산",
    Lava: "용암",
    Slime: "슬라임",
    Nitro: "니트로",
    Gas: "가스",
    Steam: "증기",
    Fire: "불",
    Wall: "벽",
    Wood: "나무",
    Plant: "식물",
    Fungus: "균류",
    Seed: "씨앗",
    Metal: "금속",
    Electricity: "전기",
    Rocket: "로켓",
    Cloner: "복제기",
    Mite: "응애",
    Ice: "얼음",
  },
  ja: {
    Sand: "砂",
    Dust: "ダスト",
    Stone: "石",
    Glass: "ガラス",
    Water: "水",
    Oil: "油",
    Acid: "酸",
    Lava: "溶岩",
    Slime: "スライム",
    Nitro: "ニトロ",
    Gas: "ガス",
    Steam: "蒸気",
    Fire: "火",
    Wall: "壁",
    Wood: "木",
    Plant: "植物",
    Fungus: "菌類",
    Seed: "種",
    Metal: "金属",
    Electricity: "電気",
    Rocket: "ロケット",
    Cloner: "クローン",
    Mite: "ダニ",
    Ice: "氷",
  },
  zh: {
    Sand: "沙子",
    Dust: "粉尘",
    Stone: "石头",
    Glass: "玻璃",
    Water: "水",
    Oil: "油",
    Acid: "酸",
    Lava: "熔岩",
    Slime: "史莱姆",
    Nitro: "硝爆液",
    Gas: "气体",
    Steam: "蒸汽",
    Fire: "火",
    Wall: "墙",
    Wood: "木头",
    Plant: "植物",
    Fungus: "真菌",
    Seed: "种子",
    Metal: "金属",
    Electricity: "电流",
    Rocket: "火箭",
    Cloner: "复制器",
    Mite: "螨虫",
    Ice: "冰",
  },
  de: {
    Sand: "Sand",
    Dust: "Staub",
    Stone: "Stein",
    Glass: "Glas",
    Water: "Wasser",
    Oil: "Öl",
    Acid: "Säure",
    Lava: "Lava",
    Slime: "Schleim",
    Nitro: "Nitro",
    Gas: "Gas",
    Steam: "Dampf",
    Fire: "Feuer",
    Wall: "Wand",
    Wood: "Holz",
    Plant: "Pflanze",
    Fungus: "Pilz",
    Seed: "Samen",
    Metal: "Metall",
    Electricity: "Elektrizität",
    Rocket: "Rakete",
    Cloner: "Kloner",
    Mite: "Milbe",
    Ice: "Eis",
  },
};

const ELEMENT_GROUPS = [
  { titleKey: "powders", names: ["Sand", "Dust", "Stone", "Glass"] },
  { titleKey: "liquids", names: ["Water", "Oil", "Acid", "Lava", "Slime", "Nitro"] },
  { titleKey: "gases", names: ["Gas", "Steam", "Fire"] },
  { titleKey: "structures", names: ["Wall", "Wood", "Plant", "Fungus", "Seed", "Metal"] },
  { titleKey: "chaos", names: ["Electricity", "Rocket", "Cloner", "Mite", "Ice"] },
];

const DISASTER_OPTIONS = [
  { id: "none", labelKey: "calm" },
  { id: "rain", labelKey: "rain" },
  { id: "typhoon", labelKey: "typhoon" },
  { id: "earthquake", labelKey: "earthquake" },
  { id: "lightning", labelKey: "lightning" },
  { id: "meteor", labelKey: "meteor" },
];

const ElementButton = (name, label, selectedElement, setElement) => {
  let elementID = Species[name];
  if (!Number.isInteger(elementID)) {
    return null;
  }

  let color = pallette_data[elementID] || "rgba(220, 220, 220, 0.25)";
  let selected = elementID == selectedElement;

  let background = "inherit";
  if (elementID == 14) {
    background = `linear-gradient(45deg, 
    rgba(202, 121, 125, 0.25), 
    rgba(169, 120, 200, 0.25), 
    rgba(117, 118, 195, 0.25), 
    rgba(117, 196, 193, 0.25), 
    rgba(122, 203, 168, 0.25), 
    rgba(185, 195, 117, 0.25), 
    rgba(204, 186, 122, 0.25))`;
    if (selected) {
      background = background.replace(/0.25/g, "1.0");
    }
  }
  return (
    <button
      className={selected ? "selected" : ""}
      key={name}
      onClick={() => {
        setElement(elementID);
      }}
      style={{
        background,
        backgroundColor: selected ? color.replace("0.25", "1.5") : color,
      }}
    >
      {"  "}
      {label}
      {"  "}
    </button>
  );
};

let sizeMap = [1, 3, 7, 19, 39];

class Index extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      submissionMenuOpen: false,
      paused: false,
      submitting: false,
      size: 2,
      dataURL: {},
      currentSubmission: null,
      selectedElement: Species.Water,
      activeDisaster: "none",
      settingsOpen: false,
      maxFps: Number(localStorage.getItem("maxFps") || 60),
      language: localStorage.getItem("language") || "en",
    };
    window.maxFps = this.state.maxFps;
    window.appLanguage = this.state.language;
    window.UI = this;
    //if we start in the background, pause;
    if (
      this.props.location.pathname !== "/" &&
      this.props.location.pathname !== "/school"
    ) {
      window.setTimeout(() => this.pause(), 50);
    }

    this.load();
  }

  componentDidUpdate(prevProps) {
    if (
      this.props.location.pathname === "/" &&
      prevProps.location.pathname !== "/" &&
      this.state.currentSubmission
    ) {
      window.location = `#${this.state.currentSubmission.id}`;
      return;
    }
    if (
      this.props.location.pathname !== "/" &&
      prevProps.location.pathname == "/"
    ) {
      this.pause();
    }
    if (
      prevProps.location.hash === "" ||
      prevProps.location.hash != this.props.location.hash
    ) {
      this.load();
    }
  }
  togglePause() {
    window.paused = !this.state.paused;
    this.setState({ paused: !this.state.paused });
  }
  play() {
    window.paused = false;
    this.setState({ paused: false });
  }
  pause() {
    window.paused = true;
    this.setState({ paused: true });
  }

  setSize(event, size) {
    event.preventDefault();
    this.setState({
      size,
    });
  }
  setDisaster(activeDisaster) {
    this.setState({ activeDisaster });
  }
  openSettings() {
    this.pause();
    this.setState({ settingsOpen: true });
  }
  closeSettings() {
    this.play();
    this.setState({ settingsOpen: false });
  }
  setMaxFps(maxFps) {
    const value = Number(maxFps);
    localStorage.setItem("maxFps", String(value));
    window.maxFps = value;
    this.setState({ maxFps: value });
  }
  setLanguage(language) {
    localStorage.setItem("language", language);
    window.appLanguage = language;
    this.setState({ language });
  }
  reset() {
    if (window.confirm("Are you sure you want to reset?")) {
      this.play();
      window.location = "#";
      this.setState({ currentSubmission: null });
      reset();
    }
  }
  menu() {
    this.pause();
    this.setState({ submissionMenuOpen: true });
  }

  closeMenu() {
    this.play();
    this.setState({ submissionMenuOpen: false });
  }
  upload() {
    let dataURL = snapshot(universe);
    const cells = new Uint8Array(
      memory.buffer,
      universe.cells(),
      width * height * 4
    );

    // Create canvas
    let canvas = document.createElement("canvas"),
      context = canvas.getContext("2d"),
      imgData = context.createImageData(width, height);

    canvas.height = height;
    canvas.width = width;

    // fill imgData with data from cells
    // transpose for historical compatability
    for (let x = 0; x < width; x++) {
      for (let y = 0; y < height; y++) {
        let cell_index = (y + x * height) * 4;
        let img_index = (x + y * width) * 4;
        for (var i = 0; i < 4; i++) {
          if (i % 4 == 3) {
            imgData.data[img_index + i] = 255;
          } else {
            imgData.data[img_index + i] = cells[cell_index + i];
          }
        }
      }
    }
    // put data to context at (0, 0)
    context.putImageData(imgData, 0, 0);

    let cellData = canvas.toDataURL("image/png");

    this.pause();
    this.setState({
      data: { dataURL, cells: cellData },
      submissionMenuOpen: true,
    });
  }
  rateLimited() {
    var postList = JSON.parse(localStorage.getItem("postList") || "[]");
    postList = postList.filter((post) => Date.now() - 1000 * 60 * 5 < post);

    if (postList.length >= 3) {
      Sentry.captureMessage("RATELIMIT");
      return true;
    }
    return false;
  }
  submit() {
    let { title, data, currentSubmission } = this.state;

    let { dataURL, cells } = data;
    let { currentUser } = firebase.auth();
    title = title.replace(
      "[profile]",
      `https://sandspiel.club/browse/search/?user=${currentUser.uid}`
    );

    let payload = {
      title,
      image: dataURL,
      parent_id: currentSubmission?.data?.id,
      cells,
    };

    var postList = JSON.parse(localStorage.getItem("postList") || "[]");

    postList = postList.filter((post) => Date.now() - 1000 * 60 * 3 < post);
    postList.push(Date.now());
    localStorage.setItem("postList", JSON.stringify(postList));

    this.setState({ submitting: true });
    currentUser.getIdToken().then((token) => {
      fetch(functions._url("api/creations"), {
        method: "POST",
        body: JSON.stringify(payload), // data can be `string` or {object}!
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer " + token,
        },
      })
        .then((res) => res.json())
        .then((response) => {
          console.log("Success:", JSON.stringify(response));
          this.play();
        })
        .catch((error) => console.error("Error:", error))
        .then(() => {
          this.setState({ submissionMenuOpen: false, submitting: false });
        });
    });
  }

  async loadSVG(svgString) {
    const imgData = await svgToImageData(svgString);

    const cellsData = new Uint8Array(
      memory.buffer,
      universe.cells(),
      width * height * 4
    );

    reset();
    window.stopboot = true;

    for (let i = 0, len = width * height * 4; i < len; i += 4) {
      const species = rgbaToSpecies(
        imgData.data[i],
        imgData.data[i + 1],
        imgData.data[i + 2],
        imgData.data[i + 3]
      );
      cellsData[i] = species; // should be 0 to 19
      cellsData[i + 1] = Math.floor(100 + Math.random() * 50); // register A
      cellsData[i + 2] = 0; // register B
      cellsData[i + 3] = 0; // clock
    }
    universe.flush_undos();
    safePushUndo();

    this.pause();
  }

  load() {
    let { location } = this.props;
    let id = location.hash.replace(/#/, "");
    if (id === "") {
      return;
    }

    if (this.state.currentSubmission && this.state.currentSubmission.id == id) {
      return;
    }

    fetch(functions._url(`api/creations/${id}`), {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    })
      .then((res) => res.json())
      .then((data) => {
        storage
          .refFromURL(
            `gs://sandtable-8d0f7.appspot.com/creations/${data.id}.data.png`
          )
          .getDownloadURL()
          .then((dlurl) => {
            fetch(dlurl, {
              method: "GET",
            })
              .then((res) => res.blob())
              .then((blob) => {
                this.setState({ currentSubmission: { id, data } });

                var url = URL.createObjectURL(blob);
                var img = new Image();
                img.src = url;
                img.onload = () => {
                  var canvas = document.createElement("canvas");
                  canvas.width = width;
                  canvas.height = height;
                  var ctx = canvas.getContext("2d");

                  ctx.translate(canvas.width / 2, canvas.height / 2);
                  ctx.rotate((-90 * Math.PI) / 180);
                  ctx.scale(-1, 1.0);
                  ctx.translate(-canvas.width / 2, -canvas.height / 2);

                  ctx.drawImage(img, 0, 0);
                  var imgData = ctx.getImageData(
                    0,
                    0,
                    canvas.width,
                    canvas.height
                  );

                  const cellsData = new Uint8Array(
                    memory.buffer,
                    universe.cells(),
                    width * height * 4
                  );

                  reset();
                  window.stopboot = true;

                  for (var i = 0; i < width * height * 4; i++) {
                    cellsData[i] = imgData.data[i];
                  }
                  universe.flush_undos();
                  safePushUndo();
                  this.pause();
                };
              })
              .catch((error) => console.error("Error:", error));
          });
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  }
  incScore() {
    let { currentSubmission } = this.state;
    let { id } = currentSubmission;
    // creations/:id/vote
    firebase
      .auth()
      .currentUser.getIdToken()
      .then((token) => {
        fetch(functions._url(`api/creations/${id}/vote`), {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
            Authorization: "Bearer " + token,
          },
        })
          .then((res) => res.json())
          .then((data) => {
            if (currentSubmission != null) {
              this.setState({
                currentSubmission: { ...currentSubmission, data },
              });
            }
          })
          .catch((e) => {
            console.error(e);
          });
      });
  }

  render() {
    let {
      size,
      paused,
      selectedElement,
      currentSubmission,
      activeDisaster,
      settingsOpen,
      maxFps,
      language,
    } = this.state;
    const text = UI_TEXT[language] || UI_TEXT.en;
    const elementText = ELEMENT_LABELS[language] || ELEMENT_LABELS.en;
    let hash =
      currentSubmission && currentSubmission.id
        ? `#${currentSubmission.id}`
        : "";
    return (
      <React.Fragment>
        <Promotab />
        <button
          onClick={() => this.togglePause()}
          className={paused ? "selected" : ""}
        >
          {paused ? (
            <svg height="20" width="20" id="d" viewBox="0 0 300 300">
              <polygon id="play" points="0,0 , 300,150 0,300" />
            </svg>
          ) : (
            <svg height="20" width="20" id="d" viewBox="0 0 300 300">
              <polygon id="bar2" points="0,0 110,0 110,300 0,300" />
              <polygon id="bar1" points="190,0 300,0 300,300 190,300" />
            </svg>
          )}
        </button>

        {!window.location.pathname.includes("school") && (
          <>
            <button onClick={() => this.openSettings()}>{text.settings}</button>
            <Link
              to={{
                pathname: "/browse/",
                hash,
              }}
            >
              <button>{text.browse}</button>
            </Link>
          </>
        )}

        <button onClick={() => this.reset()}>{text.reset}</button>
        <Link
          to={{
            pathname: "/info/",
            hash,
          }}
        >
          <button>{text.info}</button>
        </Link>

        {/* {paused && <button onClick={() => universe.tick()}>Tick</button>} */}
        <span className="sizes">
          {sizeMap.map((v, i) => (
            <button
              key={i}
              className={i == size ? "selected" : ""}
              onClick={(e) => this.setSize(e, i)}
              style={{ padding: "0px" }}
            >
              <svg height="23" width="23" id="d" viewBox="0 0 100 100">
                <circle cx="50" cy="50" r={3 + v} />
              </svg>
            </button>
          ))}
        </span>
        <button
          onClick={() => {
            reset();
            universe.pop_undo();
          }}
          style={{ fontSize: 35 }}
        >
          ↜
        </button>
        <button
          className={-1 == selectedElement ? "selected" : ""}
          key={name}
          onClick={() => {
            this.setState({ selectedElement: -1 });
          }}
        >
          {text.wind}
        </button>
        {ELEMENT_GROUPS.map((group) => (
          <div className="tool-section" key={group.titleKey}>
            <div className="tool-section-title">{text[group.titleKey]}</div>
            {group.names
              .filter((name) => typeof Species[name] === "number")
              .map((n) =>
                ElementButton(n, elementText[n] || n, selectedElement, (id) =>
                  this.setState({ selectedElement: id })
                )
              )}
          </div>
        ))}
        <div className="tool-section disaster-section">
          <div className="tool-section-title">{text.naturalDisasters}</div>
          {DISASTER_OPTIONS.map((option) => (
            <button
              key={option.id}
              className={activeDisaster === option.id ? "selected" : ""}
              onClick={() => this.setDisaster(option.id)}
            >
              {text[option.labelKey]}
            </button>
          ))}
        </div>
        {/* <span className="promo">
          *new*{" "}
          <a href="https://orb.farm" target="_blank">
            orb.farm
          </a>
        </span> */}
        {this.state.currentSubmission && (
          <div className="submission-title">
            <button onClick={() => this.incScore()}>
              +♡{this.state.currentSubmission.data.score}{" "}
            </button>
            {this.state.currentSubmission.data.title}
          </div>
        )}

        {this.state.submissionMenuOpen && (
          <Menu close={() => this.closeMenu()}>
            <h4>Share your creation with the people! (try using #hashtags)</h4>
            <p>
              Please be nice. Users who post hateful or sexually explicit
              content will be banned.
            </p>
            <img src={this.state.data.dataURL} className="submissionImg" />
            <SignInButton>
              <div style={{ display: "flex" }}>
                <input
                  maxlength="200"
                  placeholder="Title"
                  onChange={(e) => this.setState({ title: e.target.value })}
                />
                <button
                  disabled={this.state.submitting || this.rateLimited()}
                  onClick={() => this.submit()}
                >
                  Submit
                </button>
              </div>
            </SignInButton>
          </Menu>
        )}

        {settingsOpen && (
          <Menu close={() => this.closeSettings()}>
            <div className="settings-panel">
              <h2>{text.settingsTitle}</h2>
              <label className="settings-row">
                <span>{text.maxFps}</span>
                <select
                  value={String(maxFps)}
                  onChange={(e) => this.setMaxFps(e.target.value)}
                >
                  <option value="30">30</option>
                  <option value="45">45</option>
                  <option value="60">60</option>
                  <option value="90">90</option>
                  <option value="120">120</option>
                  <option value="0">{text.unlimited}</option>
                </select>
              </label>
              <label className="settings-row">
                <span>{text.language}</span>
                <select
                  value={language}
                  onChange={(e) => this.setLanguage(e.target.value)}
                >
                  <option value="en">{text.langEn}</option>
                  <option value="ko">{text.langKo}</option>
                  <option value="ja">{text.langJa}</option>
                  <option value="zh">{text.langZh}</option>
                  <option value="de">{text.langDe}</option>
                </select>
              </label>
            </div>
          </Menu>
        )}
      </React.Fragment>
    );
  }
}

export { sizeMap, Index };
