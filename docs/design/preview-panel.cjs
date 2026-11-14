// Design preview only. No changes to the plug-in renderer or hit targets.
const fs = require('fs');
const path = require('path');
const svg = [];
const add = s => svg.push(s);
const rect = (x,y,w,h,r,fill,extra='') => add(`<rect x="${x}" y="${y}" width="${w}" height="${h}" rx="${r}" fill="${fill}" ${extra}/>`);
const text = (x,y,label,size=17,fill='#e7e9e8',extra='') => add(`<text x="${x}" y="${y}" text-anchor="middle" font-family="Arial, sans-serif" font-size="${size}" fill="${fill}" ${extra}>${label}</text>`);
add(`<svg xmlns="http://www.w3.org/2000/svg" width="1500" height="920" viewBox="0 0 1500 920">
<defs>
 <linearGradient id="rim" x2=".3" y2="1"><stop stop-color="#a3a5a5"/><stop offset=".06" stop-color="#444748"/><stop offset=".5" stop-color="#17191b"/><stop offset=".94" stop-color="#65696a"/><stop offset="1" stop-color="#2e3032"/></linearGradient>
 <linearGradient id="case" x2=".6" y2="1"><stop stop-color="#303235"/><stop offset=".48" stop-color="#222427"/><stop offset="1" stop-color="#1b1d20"/></linearGradient>
 <linearGradient id="tray" x2=".15" y2="1"><stop stop-color="#111214"/><stop offset=".08" stop-color="#35383a"/><stop offset="1" stop-color="#202225"/></linearGradient>
 <linearGradient id="edge" x2=".7" y2="1"><stop stop-color="#74797c"/><stop offset=".16" stop-color="#393c40"/><stop offset=".6" stop-color="#17191c"/><stop offset="1" stop-color="#08090a"/></linearGradient>
 <linearGradient id="rubber" x2=".25" y2="1"><stop stop-color="#393d41"/><stop offset=".48" stop-color="#292c30"/><stop offset="1" stop-color="#202327"/></linearGradient>
 <radialGradient id="knob"><stop stop-color="#303338"/><stop offset=".75" stop-color="#25282c"/><stop offset=".94" stop-color="#191b1e"/><stop offset="1" stop-color="#45494d"/></radialGradient>
 <linearGradient id="glass" x2="1" y2="1"><stop stop-color="#26292c"/><stop offset=".46" stop-color="#08090a"/><stop offset="1" stop-color="#101214"/></linearGradient>
 <linearGradient id="lcd" x2="0" y2="1"><stop stop-color="#b4d4df"/><stop offset="1" stop-color="#8bb5c8"/></linearGradient>
 <filter id="shadow" x="-40%" y="-40%" width="180%" height="190%"><feDropShadow dx="2" dy="5" stdDeviation="3" flood-color="#000" flood-opacity=".8"/></filter>
 <filter id="grain"><feTurbulence type="fractalNoise" baseFrequency=".7" numOctaves="3" stitchTiles="stitch"/><feColorMatrix type="saturate" values="0"/><feComponentTransfer><feFuncA type="linear" slope=".055"/></feComponentTransfer><feBlend in="SourceGraphic" mode="soft-light"/></filter>
</defs>`);
rect(0,0,1500,920,0,'#141619');
rect(13,16,1474,889,57,'#08090a','filter="url(#shadow)"');
rect(10,9,1480,887,57,'url(#rim)');
rect(17,16,1466,873,51,'url(#case)','stroke="#55585a" stroke-width="1.2"');
rect(22,21,1456,863,47,'url(#case)','filter="url(#grain)"');
add('<path d="M66 23 H1430 Q1474 23 1475 71" fill="none" stroke="#8f9498" opacity=".19"/>');
function knob(x,y,label,angle=0){
 text(x,y-69,label,20,'#dedfdc','letter-spacing=".8"');
 add(`<g transform="translate(${x} ${y})" filter="url(#shadow)">`);
 add('<circle r="37" fill="#090a0c" stroke="#414549" stroke-width="1.2"/><circle r="32" fill="url(#edge)"/>');
 for(let i=0;i<36;i++){const a=i*Math.PI/18;const xx=Math.sin(a), yy=Math.cos(a);add(`<path d="M${xx*29} ${yy*29} L${xx*32} ${yy*32}" stroke="${i<18?'#9b9da0':'#55595e'}" stroke-width="1.7"/>`);}
 add('<circle r="26" fill="url(#knob)" stroke="#0c0e10"/>');
 add(`<path d="M0 -20 L0 -27" stroke="#eeeeea" stroke-width="4" stroke-linecap="round" transform="rotate(${angle})"/>`);
 add('<path d="M-23 -7 A24 24 0 0 1 9 -23" fill="none" stroke="#a7adb2" opacity=".2"/>');
 add('</g>');
}
knob(126,155,'MASTER',-15);knob(281,155,'SELECT');knob(126,312,'PRESETS');knob(281,312,'ALGORITHM');
for(let i=0;i<4;i++)knob(839+i*169,155,`KNOB${i+1}`);
// Glass enclosure and an illustrative firmware screen, not a captured running UI.
rect(389,80,353,361,40,'#050607','stroke="#414548" stroke-width="1.5"');
rect(392,83,347,355,38,'url(#glass)');
add('<path d="M407 86 H655 L397 343 V107 Q397 87 407 86" fill="#fff" opacity=".025"/>');
rect(422,121,286,267,6,'#0a0e11','stroke="#020303" stroke-width="5"');
add('<g id="sample-screen">');
rect(424,123,282,263,4,'url(#lcd)','stroke="#d0e8ed" stroke-width="1.5"');
text(535,155,'001 PIANO 1',23,'#203640');
rect(668,136,24,22,2,'#e3edf0');rect(674,140,4,14,0,'#27353c');rect(682,140,4,14,0,'#27353c');
add('<path d="M429 171 H701 M429 340 H701" stroke="#608e9e" opacity=".8"/>');
['Filter','Reverb','Delay','Distort','Chorus','Phaser'].forEach((s,i)=>{const x=432+(i%3)*89,y=180+Math.floor(i/3)*76;rect(x,y,81,58,3,'#87aabd','stroke="#d0e2e7"');rect(x+2,y+2,77,54,2,'#8aaebf','stroke="#668c9c"');text(x+40,y+36,s,19,'#233c48');});
add('<circle cx="472" cy="247" r="4.8" fill="#ea5d35"/>');
rect(445,349,241,29,3,'#d2e7ea','stroke="#638e9d"');text(565,370,'Type: LOW PASS',21,'#273d46');
add('</g>');
function tray(x,y,w,h,r=23){rect(x,y,w,h,r,'#08090a','stroke="#82878a" stroke-opacity=".5"');rect(x+3,y+3,w-6,h-6,r-2,'url(#tray)','stroke="#090a0b" stroke-width="2"');}
function button(x,y,w,h,label){
 rect(x,y,w,h,12,'#080a0b','filter="url(#shadow)"');
 rect(x+2,y+1,w-4,h-3,11,'url(#edge)','stroke="#777d81" stroke-opacity=".25"');
 rect(x+6,y+6,w-12,h-13,8,'url(#rubber)','stroke="#0e1012" stroke-width="1.4"');
 add(`<path d="M${x+12} ${y+8} H${x+w-15}" stroke="#a9aeb2" stroke-opacity=".16"/>`);
 if(label==='PLAY/STOP'){text(x+w/2,y+h/2-6,'PLAY',17,'#eff0ec','font-weight="bold"');add(`<path d="M${x+17} ${y+h/2} H${x+w-17}" stroke="#dadcd9"/>`);text(x+w/2,y+h/2+20,'STOP',17,'#eff0ec','font-weight="bold"');}
 else text(x+w/2,y+h/2+6,label,17,'#eff0ec','font-weight="bold" letter-spacing=".5"');
}
tray(91,397,235,74,15);button(101,406,99,53,'OCT−');button(214,406,101,53,'OCT+');
tray(792,264,603,212);
['FX','SEL','ENV','LFO','EDIT','GLO','HOME','SAVE','ARP','SEQ','PLAY/STOP','REC'].forEach((s,i)=>button(813+i%6*95,284+Math.floor(i/6)*91,79,78,s));
add('<circle cx="1093" cy="370" r="4" fill="#0b0d0e" stroke="#494d50"/>');
// Retain plug-in settings as a quiet, matching panel key.
button(1264,489,121,42,'MENU');
text(750,510,"F’Em 1",15,'#8d9397','letter-spacing="3"');
tray(58,545,1383,304,43);
const labels=['OP1','OP2','OP3','OP4','OP5','OP6','PIT','GLO','MONO','POLY',''];
function key(x,y,w,h,label){
 rect(x,y,w,h,w/2,'#101214','filter="url(#shadow)"');
 rect(x+2,y,w-4,h-3,w/2,'url(#edge)','stroke="#646a6e" stroke-opacity=".42"');
 rect(x+7,y+7,w-14,h-16,(w-14)/2,'url(#rubber)','stroke="#121416" stroke-width="1.5"');
 const ridgeH=label?39:73;
 rect(x+w/2-4,y+34,8,ridgeH,4,'#080a0b','stroke="#555c61"');
 rect(x+w/2-2.7,y+35,5.4,ridgeH-2,2.7,'#eceeeb');
 if(label)text(x+w/2,y+98,label,13,'#e5e8e5','font-weight="bold" letter-spacing=".7"');
 add(`<path d="M${x+9} ${y+28} Q${x+14} ${y+6} ${x+w/2} ${y+6}" stroke="#a5adb4" stroke-opacity=".22" fill="none"/>`);
}
const upper=[123,207,291,460,544,713,797,881,1050,1134,1303];
upper.forEach((x,i)=>key(x,565,72,133,labels[i]));
for(let i=0;i<16;i++)key(80+i*84,706,72,130,'');
for(let i=0;i<4;i++)add(`<circle cx="${414+i*253}" cy="704" r="4" fill="#0d0f11" stroke="#4c5256" stroke-width="1"/>`);
text(750,871,'FIRMWARE-POWERED FM SYNTHESIZER',10,'#666d72','letter-spacing="2.5"');
add('</svg>');
const out=path.join(__dirname,'realistic-panel-preview.svg');fs.writeFileSync(out,svg.join('\n'));
if(process.argv.includes('--png')){
 const {Resvg}=require('../../target/ui-preview/node_modules/@resvg/resvg-js');
 fs.writeFileSync(path.join(__dirname,'../../target/ui-preview/realistic-panel-preview.png'),new Resvg(svg.join('\n'),{font:{loadSystemFonts:true}}).render().asPng());
}
console.log(out);
if(process.argv.includes('--assets')) (async()=>{
 const {Resvg}=require('../../target/ui-preview/node_modules/@resvg/resvg-js');
 const sharp=require('../../target/ui-preview/node_modules/sharp');
 const dir=path.join(__dirname,'../../crates/fm1-plugin/resources/panel'); fs.mkdirSync(dir,{recursive:true});
 const themes={black:[],green:[['#393d41','#afd1b6'],['#292c30','#90b49a'],['#202327','#71947b']],orange:[['#303235','#ed8a3d'],['#222427','#d77731'],['#1b1d20','#b65a24']],purple:[['#303235','#a99ac7'],['#222427','#9180b1'],['#1b1d20','#746192'],['#393d41','#d2c6e3'],['#292c30','#b4a4cc'],['#202327','#9987b3']],white:[['#303235','#e1e5e6'],['#222427','#cbd1d3'],['#1b1d20','#a5afb4'],['#393d41','#8eacc8'],['#292c30','#7192b3'],['#202327','#537493']]};
 for(const [name,replacements] of Object.entries(themes)){
  let source=svg.join('\n').replace(/<g id="sample-screen">[\s\S]*?<\/g>/,'<rect x="424" y="123" width="282" height="263" fill="#000"/>');
  for(const [from,to] of replacements) source=source.replaceAll(from,to);
  if(name==='green'||name==='purple') for(const ink of ['#eff0ec','#e5e8e5'])source=source.replaceAll(ink,name==='green'?'#263d2d':'#352744');
  if(['orange','purple','white'].includes(name)) source=source.replaceAll('#dedfdc','#303338');
  const png=new Resvg(source,{font:{loadSystemFonts:true}}).render().asPng();
  await sharp(png).png({palette:true,colours:128,dither:0,compressionLevel:9}).toFile(path.join(dir,`${name}.png`));
 }
})().catch(e=>{console.error(e);process.exitCode=1;});
