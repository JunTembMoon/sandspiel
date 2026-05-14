import React from "react";

const INFO_TEXT = {
  en: {
    originalBy: "Original game by",
    modifiedBy: "Modified by rove",
    intro1:
      "I'm rove, a middle school student living in Korea. I first tried this game after noticing some classmates secretly playing it during class, and it was so fun that I wanted to keep expanding it with more elements, disasters, and weird interactions.",
    intro2:
      "I hope you enjoy this version and have a good time experimenting with it. Below is the original introduction from the original creator:",
    elementInfo: "Element Information",
    elements: {
      Wall: "Indestructible.",
      Sand: "Sinks in water.",
      Water: "Puts out fire.",
      Stone: "Forms arches and turns into sand under pressure.",
      Ice: "Freezes water and feels slippery.",
      Gas: "Highly flammable.",
      Cloner: "Copies the first element it touches.",
      Mite: "Eats wood and plant, but loves dust.",
      Wood: "Sturdy, but biodegradable.",
      Plant: "Thrives in wet environments.",
      Fungus: "Spreads over almost everything.",
      Seed: "Grows on sand, plant, and fungus.",
      Fire: "Hot.",
      Lava: "Flammable and heavy.",
      Acid: "Corrodes other elements.",
      Dust: "Pretty, but dangerously explosive.",
      Oil: "Produces smoke when set on fire.",
      Rocket: "Explodes into copies of the first element it touches.",
      Empty: "Erases.",
    },
  },
  ko: {
    originalBy: "원본 게임 제작자",
    modifiedBy: "수정자 rove",
    intro1:
      "나는 한국에 사는 중학생 rove다. 반 친구들이 수업 시간에 몰래 이 게임을 하는 걸 보고 따라 해봤는데 너무 재미있어서 요소와 재해, 이상한 상호작용을 더 넣고 싶어졌다.",
    intro2:
      "이 버전도 재미있게 즐기길 바란다. 아래는 원본 제작자의 소개글이다:",
    elementInfo: "요소 정보",
    elements: {
      Wall: "파괴되지 않는다.",
      Sand: "물속으로 가라앉는다.",
      Water: "불을 끈다.",
      Stone: "아치를 만들고 압력을 받으면 모래가 된다.",
      Ice: "물을 얼리고 미끄럽다.",
      Gas: "매우 잘 탄다.",
      Cloner: "처음 닿은 요소를 복제한다.",
      Mite: "나무와 식물을 먹지만 먼지를 특히 좋아한다.",
      Wood: "튼튼하지만 결국 분해된다.",
      Plant: "축축한 환경에서 잘 자란다.",
      Fungus: "거의 모든 곳으로 퍼진다.",
      Seed: "모래, 식물, 균류 위에서 자란다.",
      Fire: "뜨겁다.",
      Lava: "잘 타고 무겁다.",
      Acid: "다른 요소를 부식시킨다.",
      Dust: "예쁘지만 매우 폭발적이다.",
      Oil: "불이 붙으면 연기를 낸다.",
      Rocket: "처음 닿은 요소의 복사본으로 폭발한다.",
      Empty: "지운다.",
    },
  },
  ja: {
    originalBy: "原作ゲーム制作",
    modifiedBy: "改造版 rove",
    intro1:
      "私は韓国に住んでいる中学生のroveです。クラスメートが授業中にこっそりこのゲームで遊んでいるのを見て自分も試してみたら、とても面白くて、もっと多くの要素や災害、変な相互作用を追加したくなりました。",
    intro2:
      "このバージョンも楽しく遊んでくれたらうれしいです。以下は原作者の紹介文です。",
    elementInfo: "要素情報",
    elements: {
      Wall: "壊れません。",
      Sand: "水の中に沈みます。",
      Water: "火を消します。",
      Stone: "アーチを作り、圧力で砂になります。",
      Ice: "水を凍らせ、滑りやすいです。",
      Gas: "とても燃えやすいです。",
      Cloner: "最初に触れた要素を複製します。",
      Mite: "木や植物を食べ、特にダストが好きです。",
      Wood: "丈夫ですが分解されます。",
      Plant: "湿った環境でよく育ちます。",
      Fungus: "ほとんど何にでも広がります。",
      Seed: "砂、植物、菌類の上で育ちます。",
      Fire: "熱いです。",
      Lava: "燃えやすく重いです。",
      Acid: "他の要素を腐食させます。",
      Dust: "きれいですが危険なほど爆発的です。",
      Oil: "燃えると煙を出します。",
      Rocket: "最初に触れた要素のコピーとして爆発します。",
      Empty: "消去します。",
    },
  },
  zh: {
    originalBy: "原作游戏作者",
    modifiedBy: "修改版 rove",
    intro1:
      "我是住在韩国的中学生 rove。看到班里的同学上课时偷偷玩这个游戏后，我也试了一下，结果觉得太有意思了，所以想继续往里面加更多元素、灾害和奇怪的互动。",
    intro2:
      "希望你也能喜欢这个版本。下面是原作者的介绍文：",
    elementInfo: "元素信息",
    elements: {
      Wall: "不可破坏。",
      Sand: "会沉入水中。",
      Water: "可以灭火。",
      Stone: "会形成拱形，受压后会变成沙子。",
      Ice: "会冻结水，而且很滑。",
      Gas: "非常易燃。",
      Cloner: "复制它接触到的第一个元素。",
      Mite: "会吃木头和植物，尤其喜欢粉尘。",
      Wood: "结实，但会被分解。",
      Plant: "在潮湿环境里生长良好。",
      Fungus: "会蔓延到几乎所有地方。",
      Seed: "在沙子、植物和真菌上生长。",
      Fire: "很热。",
      Lava: "可燃而且沉重。",
      Acid: "会腐蚀其他元素。",
      Dust: "很漂亮，但爆炸性很强。",
      Oil: "着火时会产生烟雾。",
      Rocket: "会爆炸成它接触到的第一个元素的复制体。",
      Empty: "擦除。",
    },
  },
  de: {
    originalBy: "Originalspiel von",
    modifiedBy: "Modifiziert von rove",
    intro1:
      "Ich bin rove, ein Mittelschüler aus Korea. Ich habe dieses Spiel ausprobiert, nachdem ich gesehen habe, wie Mitschüler es heimlich im Unterricht gespielt haben, und es hat so viel Spaß gemacht, dass ich mehr Elemente, Katastrophen und seltsame Wechselwirkungen hinzufügen wollte.",
    intro2:
      "Ich hoffe, dir gefällt diese Version. Unten steht die ursprüngliche Vorstellung des Originalautors:",
    elementInfo: "Elementinformationen",
    elements: {
      Wall: "Unzerstörbar.",
      Sand: "Sinkt im Wasser.",
      Water: "Löscht Feuer.",
      Stone: "Bildet Bögen und wird unter Druck zu Sand.",
      Ice: "Friert Wasser ein und ist rutschig.",
      Gas: "Sehr leicht entzündlich.",
      Cloner: "Kopiert das erste Element, das es berührt.",
      Mite: "Frisst Holz und Pflanzen, liebt aber Staub.",
      Wood: "Stabil, aber biologisch abbaubar.",
      Plant: "Wächst gut in feuchten Umgebungen.",
      Fungus: "Breitet sich fast überall aus.",
      Seed: "Wächst auf Sand, Pflanzen und Pilzen.",
      Fire: "Heiß.",
      Lava: "Brennbar und schwer.",
      Acid: "Zersetzt andere Elemente.",
      Dust: "Hübsch, aber gefährlich explosiv.",
      Oil: "Erzeugt Rauch, wenn es brennt.",
      Rocket: "Explodiert in Kopien des ersten Elements, das es berührt.",
      Empty: "Löscht.",
    },
  },
};

const ORIGINAL_CREATOR_TEXT = {
  en: {
    p1: "Welcome, and thanks for coming by! I hope that you enjoy exploring this small game, and it brings you some calm.",
    p2: "Growing up, \"falling sand\" games like this one provided me hours of entertainment and imagination. I want to particularly thank ha55ii's Powder Game as the primary inspiration for sandspiel.",
    p3: "If you want to read more the inspiration, architecture, and history of the game, I wrote a blog post (it gets technical in the middle):",
    p4: "If you'd like, you can view the source code or report bugs on github or feel free to reach out on twitter and I'll try to answer!",
    p5: "Lastly, I want to say that if you enjoy this game or share your artwork on it, your opinion is important to me and I want to do my best to ensure sandspiel is a friendly and kind place to play, without bullying, racism, transphobia, homophobia, or any other forms of bigotry. If something is wrong or there's some way I can help, feel free to contact me at",
  },
  ko: {
    p1: "와줘서 고맙습니다. 이 작은 게임을 탐험하면서 조금이라도 편안함을 느끼길 바랍니다.",
    p2: "어릴 때 이런 \"falling sand\" 게임들은 제게 오랜 즐거움과 상상력을 주었습니다. 특히 sandspiel의 가장 큰 영감이 된 ha55ii의 Powder Game에 감사하고 싶습니다.",
    p3: "게임의 영감, 구조, 역사에 대해 더 읽고 싶다면 제가 쓴 블로그 글이 있습니다(중간부터는 기술적인 내용도 있습니다):",
    p4: "원한다면 소스 코드를 보거나 github에서 버그를 제보할 수 있고, twitter로 연락해 주면 답변해 보겠습니다!",
    p5: "마지막으로, 이 게임을 즐기거나 작품을 공유해 준다면 그 의견은 제게 중요하며, 괴롭힘, 인종차별, 트랜스혐오, 동성애혐오 등 어떤 형태의 혐오도 없는 친절한 공간이 되도록 최선을 다하고 싶습니다. 문제가 있거나 제가 도울 수 있는 일이 있다면",
  },
  ja: {
    p1: "来てくれてありがとうございます。この小さなゲームを楽しみながら、少しでも穏やかな気持ちになってくれたらうれしいです。",
    p2: "子どもの頃、このような \"falling sand\" ゲームは何時間もの楽しさと想像力を与えてくれました。特に sandspiel の主な着想源となった ha55ii の Powder Game に感謝しています。",
    p3: "このゲームの着想、構造、歴史についてもっと読みたい方のために、ブログ記事を書きました（途中から少し技術的になります）:",
    p4: "よければソースコードを見たり、githubでバグを報告したり、twitterで連絡してください。できるだけ答えます。",
    p5: "最後に、このゲームを楽しんだり作品を共有してくれるなら、その意見はとても大切です。いじめ、人種差別、トランスフォビア、ホモフォビアなど、あらゆる偏見のない優しい場所にしたいと思っています。何か問題がある、あるいは私にできることがあるなら、",
  },
  zh: {
    p1: "欢迎来到这里，也感谢你的到来。希望你在探索这个小游戏时能感到轻松和平静。",
    p2: "小时候，这类 \"falling sand\" 游戏给了我很多乐趣和想象空间。我尤其想感谢 ha55ii 的 Powder Game，它是 sandspiel 最主要的灵感来源。",
    p3: "如果你想了解这款游戏的灵感、架构和历史，我写过一篇博客文章（中间部分会稍微技术一些）：",
    p4: "如果你愿意，可以查看源代码、在 github 上报告问题，或者在 twitter 上联系我，我会尽量回复。",
    p5: "最后我想说，如果你喜欢这个游戏或在这里分享作品，你的意见对我非常重要。我会尽力让 sandspiel 成为一个友善、包容、没有霸凌、种族歧视、跨性别歧视、恐同和其他偏见的地方。如果有什么问题，或者我能帮上什么忙，请联系",
  },
  de: {
    p1: "Willkommen, und danke fürs Vorbeischauen! Ich hoffe, du hast Freude daran, dieses kleine Spiel zu erkunden, und es bringt dir etwas Ruhe. ",
    p2: "Als ich aufwuchs, haben mir \"falling sand\"-Spiele wie dieses viele Stunden Unterhaltung und Fantasie geschenkt. Besonders danken möchte ich ha55iis Powder Game als wichtigste Inspiration für sandspiel.",
    p3: "Wenn du mehr über Inspiration, Architektur und Geschichte des Spiels lesen möchtest, habe ich einen Blogbeitrag geschrieben (in der Mitte wird es etwas technischer):",
    p4: "Wenn du möchtest, kannst du den Quellcode ansehen oder auf github Fehler melden, oder dich auf twitter melden, und ich versuche zu antworten!",
    p5: "Zum Schluss möchte ich sagen: Wenn dir dieses Spiel gefällt oder du deine Kunstwerke hier teilst, ist mir deine Meinung wichtig. Ich möchte mein Bestes tun, damit sandspiel ein freundlicher und respektvoller Ort bleibt, ohne Mobbing, Rassismus, Transfeindlichkeit, Homophobie oder andere Formen von Vorurteilen. Wenn etwas nicht stimmt oder ich helfen kann, kontaktiere mich unter",
  },
};

const Info = () => {
  const language = window.appLanguage || localStorage.getItem("language") || "en";
  const text = INFO_TEXT[language] || INFO_TEXT.en;
  const original = ORIGINAL_CREATOR_TEXT[language] || ORIGINAL_CREATOR_TEXT.en;
  const elementOrder = [
    "Wall",
    "Sand",
    "Water",
    "Stone",
    "Ice",
    "Gas",
    "Cloner",
    "Mite",
    "Wood",
    "Plant",
    "Fungus",
    "Seed",
    "Fire",
    "Lava",
    "Acid",
    "Dust",
    "Oil",
    "Rocket",
    "Empty",
  ];

  return (
    <div className="Info">
      <h1>Sandspiel+</h1>
      <p>
        {text.originalBy} <a href="https://maxbittker.com">max bittker</a>
      </p>
      <p>{text.modifiedBy}</p>
      <hr />
      <br />
      <p>{text.intro1}</p>
      <p>{text.intro2}</p>
      <br />
      <hr />
      <br />
      <p>{original.p1}</p>
      <p>
        {original.p2.split("Powder Game")[0]}
        <a href="https://dan-ball.jp/en/javagame/dust/">Powder Game</a> as the
        {language === "ko"
          ? " 주요 영감으로 준 ha55ii에게 특히 감사하고 싶습니다."
          : language === "ja"
            ? " を主な着想源としてくれた ha55ii に特に感謝しています。"
            : language === "zh"
              ? "，它是 sandspiel 最主要的灵感来源。"
              : language === "de"
                ? " als wichtigste Inspiration für sandspiel."
                : " primary inspiration for sandspiel."}
      </p>
      <br />
      <p>
        {original.p3}&nbsp;
        <a href="https://maxbittker.com/making-sandspiel">Making Sandspiel</a>
      </p>
      <br />
      <p>
        {original.p4.split("source code")[0]}
        <a href="https://github.com/maxbittker/sandspiel">source code</a> or{" "}
        <a href="https://github.com/maxbittker/sandspiel/issues">report bugs</a>
        {language === "ko"
          ? " 를 github에서 확인할 수 있고, twitter로 연락해 주면 답해 보겠습니다!"
          : language === "ja"
            ? " を github で確認したり、twitter で連絡してください。できるだけ返事します。"
            : language === "zh"
              ? "，也可以在 twitter 上联系我，我会尽量回复。"
              : language === "de"
                ? " auf github ansehen oder mich auf twitter kontaktieren, und ich versuche zu antworten!"
                : " on github or feel free to reach out on twitter and I'll try to answer!"}
      </p>
      <br />
      <p>
        {original.p5} <a href="mailto:maxbittker@gmail.com">maxbittker@gmail.com</a> or{" "}
        <a href="https://twitter.com/maxbittker">@maxbittker on twitter.</a>
      </p>
      <br />
      <hr />
      <br />
      <h2>{text.elementInfo}:</h2>
      {elementOrder.map((name) => (
        <React.Fragment key={name}>
          <h4>{name} </h4>
          {text.elements[name]}
        </React.Fragment>
      ))}
      <hr />
      <hr />
      <hr />
      <hr />
    </div>
  );
};

export default Info;
