import{u as R,d as G,c as g,h as E,r as I,o as T,a as W,b as J,e as K,f as Q,g as $,i as X,j as Y,p as Z,k as ee,n as te,l as ae,m as H,q as re,w as oe,s as se,t as le,v as ne,x as h,y as x,z as ie,A as ue,B as ce,C as de,D as fe,__tla as pe}from"./entry.EFwOw6dv.js";import{u as ve,__tla as he}from"./vue.f36acd1f.mPJKUFGl.js";import{_ as me}from"./_plugin-vue_export-helper.DlAUqK2U.js";let M,ge=Promise.all([(()=>{try{return pe}catch{}})(),(()=>{try{return he}catch{}})()]).then(async()=>{async function _(t,a=R()){const{path:l,matched:e}=a.resolve(t);if(!e.length||(a._routePreloaded||(a._routePreloaded=new Set),a._routePreloaded.has(l)))return;const o=a._preloadPromises=a._preloadPromises||[];if(o.length>4)return Promise.all(o).then(()=>_(t,a));a._routePreloaded.add(l);const i=e.map(u=>{var r;return(r=u.components)==null?void 0:r.default}).filter(u=>typeof u=="function");for(const u of i){const r=Promise.resolve(u()).catch(()=>{}).finally(()=>o.splice(o.indexOf(r)));o.push(r)}await Promise.all(o)}const O=(...t)=>t.find(a=>a!==void 0);function U(t){const a=t.componentName||"NuxtLink";function l(e,o){if(!e||t.trailingSlash!=="append"&&t.trailingSlash!=="remove")return e;if(typeof e=="string")return w(e,t.trailingSlash);const i="path"in e?e.path:o(e).path;return{...e,name:void 0,path:w(i,t.trailingSlash)}}return G({name:a,props:{to:{type:[String,Object],default:void 0,required:!1},href:{type:[String,Object],default:void 0,required:!1},target:{type:String,default:void 0,required:!1},rel:{type:String,default:void 0,required:!1},noRel:{type:Boolean,default:void 0,required:!1},prefetch:{type:Boolean,default:void 0,required:!1},noPrefetch:{type:Boolean,default:void 0,required:!1},activeClass:{type:String,default:void 0,required:!1},exactActiveClass:{type:String,default:void 0,required:!1},prefetchedClass:{type:String,default:void 0,required:!1},replace:{type:Boolean,default:void 0,required:!1},ariaCurrentValue:{type:String,default:void 0,required:!1},external:{type:Boolean,default:void 0,required:!1},custom:{type:Boolean,default:void 0,required:!1}},setup(e,{slots:o}){const i=R(),u=ae(),r=g(()=>{const s=e.to||e.href||"";return l(s,i.resolve)}),f=g(()=>typeof r.value=="string"&&E(r.value,{acceptRelative:!0})),A=g(()=>e.target&&e.target!=="_self"),b=g(()=>e.external||A.value?!0:typeof r.value=="object"?!1:r.value===""||f.value),B=I(!1),p=I(null),D=s=>{var c;p.value=e.custom?(c=s==null?void 0:s.$el)==null?void 0:c.nextElementSibling:s==null?void 0:s.$el};if(e.prefetch!==!1&&e.noPrefetch!==!0&&e.target!=="_blank"&&!V()){const s=H();let c,n=null;T(()=>{const y=L();W(()=>{c=J(()=>{var v;(v=p==null?void 0:p.value)!=null&&v.tagName&&(n=y.observe(p.value,async()=>{n==null||n(),n=null;const m=typeof r.value=="string"?r.value:i.resolve(r.value).fullPath;await Promise.all([s.hooks.callHook("link:prefetch",m).catch(()=>{}),!b.value&&_(r.value,i).catch(()=>{})]),B.value=!0}))})})}),K(()=>{c&&Q(c),n==null||n(),n=null})}return()=>{var v,m;if(!b.value){const d={ref:D,to:r.value,activeClass:e.activeClass||t.activeClass,exactActiveClass:e.exactActiveClass||t.exactActiveClass,replace:e.replace,ariaCurrentValue:e.ariaCurrentValue,custom:e.custom};return e.custom||(B.value&&(d.class=e.prefetchedClass||t.prefetchedClass),d.rel=e.rel||void 0),$(X("RouterLink"),d,o.default)}const s=typeof r.value=="object"?((v=i.resolve(r.value))==null?void 0:v.href)??null:r.value&&!e.external&&!f.value?l(Y(u.app.baseURL,r.value),i.resolve):r.value||null,c=e.target||null,n=O(e.noRel?"":e.rel,t.externalRelAttribute,f.value||A.value?"noopener noreferrer":"")||null,y=()=>re(s,{replace:e.replace});return e.custom?o.default?o.default({href:s,navigate:y,get route(){if(!s)return;const d=Z(s);return{path:d.pathname,fullPath:d.pathname,get query(){return ee(d.search)},hash:d.hash,params:{},name:void 0,matched:[],redirectedFrom:void 0,meta:{},href:s}},rel:n,target:c,isExternal:b.value,isActive:!1,isExactActive:!1}):null:$("a",{ref:p,href:s,rel:n,target:c},(m=o.default)==null?void 0:m.call(o))}}})}const F=U(te);function w(t,a){const l=a==="append"?oe:se;return E(t)&&!t.startsWith("http")?t:l(t,!0)}function L(){const t=H();if(t._observer)return t._observer;let a=null;const l=new Map,e=(o,i)=>(a||(a=new IntersectionObserver(u=>{for(const r of u){const f=l.get(r.target);(r.isIntersecting||r.intersectionRatio>0)&&f&&f()}})),l.set(o,i),a.observe(o),()=>{l.delete(o),a.unobserve(o),l.size===0&&(a.disconnect(),a=null)});return t._observer={observe:e}}function V(){const t=navigator.connection;return!!(t&&(t.saveData||/2g/.test(t.effectiveType)))}let S,C,k,q,P,z,N,j;S=t=>(de("data-v-05a2b8a3"),t=t(),fe(),t),C={class:"font-sans antialiased bg-white dark:bg-black text-black dark:text-white grid min-h-screen place-content-center overflow-hidden"},k=S(()=>h("div",{class:"fixed left-0 right-0 spotlight z-10"},null,-1)),q={class:"max-w-520px text-center z-20"},P=["textContent"],z=["textContent"],N={class:"w-full flex items-center justify-center"},j={__name:"error-404",props:{appName:{type:String,default:"Nuxt"},version:{type:String,default:""},statusCode:{type:Number,default:404},statusMessage:{type:String,default:"Not Found"},description:{type:String,default:"Sorry, the page you are looking for could not be found."},backHome:{type:String,default:"Go back home"}},setup(t){const a=t;return ve({title:`${a.statusCode} - ${a.statusMessage} | ${a.appName}`,script:[],style:[{children:'*,:before,:after{-webkit-box-sizing:border-box;box-sizing:border-box;border-width:0;border-style:solid;border-color:#e0e0e0}*{--tw-ring-inset:var(--tw-empty, );--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgba(14, 165, 233, .5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000}:root{-moz-tab-size:4;-o-tab-size:4;tab-size:4}a{color:inherit;text-decoration:inherit}body{margin:0;font-family:inherit;line-height:inherit}html{-webkit-text-size-adjust:100%;font-family:ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,Segoe UI,Roboto,Helvetica Neue,Arial,Noto Sans,sans-serif,"Apple Color Emoji","Segoe UI Emoji",Segoe UI Symbol,"Noto Color Emoji";line-height:1.5}h1,p{margin:0}h1{font-size:inherit;font-weight:inherit}'}]}),(l,e)=>{const o=F;return le(),ne("div",C,[k,h("div",q,[h("h1",{class:"text-8xl sm:text-10xl font-medium mb-8",textContent:x(t.statusCode)},null,8,P),h("p",{class:"text-xl px-8 sm:px-0 sm:text-4xl font-light mb-16 leading-tight",textContent:x(t.description)},null,8,z),h("div",N,[ie(o,{to:"/",class:"gradient-border text-md sm:text-xl py-2 px-4 sm:py-3 sm:px-6 cursor-pointer"},{default:ue(()=>[ce(x(t.backHome),1)]),_:1})])])])}}},M=me(j,[["__scopeId","data-v-05a2b8a3"]])});export{ge as __tla,M as default};