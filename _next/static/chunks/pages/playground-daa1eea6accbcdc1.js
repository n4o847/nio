(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[311],{8330:function(e,t,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/playground",function(){return n(9462)}])},3378:function(e,t,n){"use strict";var a=n(4246);n(7378);let r={logo:(0,a.jsx)("span",{children:"Nio"}),project:{link:"https://github.com/n4o847/nio"},toc:{extraContent:(0,a.jsx)(a.Fragment,{})},footer:{text:(0,a.jsx)(a.Fragment,{children:"\xa9 2019 n4o847"})},i18n:[],useNextSeoProps:()=>({titleTemplate:"%s – Nio"})};t.Z=r},9462:function(e,t,n){"use strict";n.r(t),n.d(t,{default:function(){return y.Z}});var a,r,s,o=n(4246),l=n(3378);n(8190);var d=n(2493),i=n(1670);n(8579);var c=n(7378),u=n(6286),g=n(7628);let f=new n.U(n(9998)),p=null;class h{static async load(){p||(p=await WebAssembly.compileStreaming(fetch(f)));let e=new g.Y;return new h({module:p,fs:e.fs})}async exec(e){let{args:t}=e,n=new u.xP({args:["nio",...t],env:{},bindings:{...u.xP.defaultBindings,path:u.xP.defaultBindings.path.default,fs:this.fs},preopens:{"/":"/"}}),a=await WebAssembly.instantiate(this.module,n.getImports(this.module));try{n.start(a)}catch(r){if(r instanceof u.CH)return r.code;throw r}}constructor({module:e,fs:t}){this.module=e,this.fs=t}}let x=()=>{let[e,t]=(0,c.useState)(null),[n,a]=(0,c.useState)(""),[r,s]=(0,c.useState)("");(0,c.useEffect)(()=>{h.load().then(e=>{t(e)})},[]),(0,c.useEffect)(()=>{e&&e.exec({args:["--version"]}).then(()=>{s([e.fs.readFileSync("/dev/stdout","utf8"),e.fs.readFileSync("/dev/stderr","utf8")].join(""))})},[e]);let l=(0,c.useCallback)(()=>{if(e)try{e.fs.appendFileSync("/dev/stdin",n),e.exec({args:["parse"]}).then(()=>{s([e.fs.readFileSync("/dev/stdout","utf8"),e.fs.readFileSync("/dev/stderr","utf8")].join(""))})}catch(t){s(e=>e+String(t)+"\n")}},[e,n]);return(0,o.jsxs)("div",{className:"py-6 w-full h-4/5 grid grid-cols-3 gap-2",children:[(0,o.jsx)("div",{className:"col-span-2 flex flex-col",children:(0,o.jsx)("textarea",{className:"px-2 py-1 border border-gray-300 dark:border-gray-500 rounded font-mono flex-grow",value:n,onChange:e=>a(e.target.value)})}),(0,o.jsxs)("div",{className:"flex flex-col gap-2",children:[(0,o.jsx)("textarea",{className:"px-2 py-1 border border-gray-300 dark:border-gray-500 rounded font-mono flex-grow bg-gray-100 dark:bg-gray-500",readOnly:!0,value:r}),(0,o.jsx)("button",{className:"p-2 bg-sky-500 hover:bg-sky-600 text-white rounded font-bold",onClick:l,children:"Parse"})]})]})};var y=n(6025);let m={filePath:"pages/playground.mdx",route:"/playground",frontMatter:{},pageMap:[{kind:"Meta",data:{index:{title:"Home",type:"page"},docs:{title:"Docs",type:"page"},playground:{title:"Playground",type:"page"}}},{kind:"Folder",name:"docs",route:"/docs",children:[{kind:"Meta",data:{start:"Get Started"}},{kind:"MdxPage",name:"start",route:"/docs/start"}]},{kind:"MdxPage",name:"index",route:"/"},{kind:"MdxPage",name:"playground",route:"/playground"}],headings:[{depth:1,value:"Nio Playground"}],flexsearch:{codeblocks:!0}},b=Symbol.for("__nextra_internal__"),j=(a=globalThis)[b]||(a[b]=Object.create(null));j.pageMap=m.pageMap,j.route=m.route,(r=j).context||(r.context=Object.create(null)),(s=j).refreshListeners||(s.refreshListeners=Object.create(null)),j.Layout=d.ZP;let v="Nio Playground";function k(e){let t=Object.assign({h1:"h1"},(0,i.ah)(),e.components);return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(t.h1,{children:"Nio Playground"}),"\n",(0,o.jsx)(x,{})]})}m.title="string"==typeof v&&v||"Playground",j.context["/playground"]={Content:function(){let e=arguments.length>0&&void 0!==arguments[0]?arguments[0]:{},{wrapper:t}=Object.assign({},(0,i.ah)(),e.components);return t?(0,o.jsx)(t,{...e,children:(0,o.jsx)(k,{...e})}):k(e)},pageOpts:m,themeConfig:l.Z}},9998:function(e,t,n){"use strict";e.exports=n.p+"static/media/nio.efd748b0.wasm"}},function(e){e.O(0,[127,170,774,888,179],function(){return e(e.s=8330)}),_N_E=e.O()}]);