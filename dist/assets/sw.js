"use strict";var e="v1.0.0::",t=["favicon.ico"];self.addEventListener("install",function(n){n.waitUntil(caches.open(e+"fundamentals").then(function(e){return e.addAll(t)}).then(function(){}))}),self.addEventListener("fetch",function(t){"GET"===t.request.method&&t.respondWith(caches.match(t.request).then(function(n){var a=fetch(t.request).then(function(n){var a=n.clone();return caches.open(e+"pages").then(function(e){e.put(t.request,a)}).then(function(){}),n},c).catch(c);return n||a;function c(){return new Response("<h1>Service Unavailable</h1>",{status:503,statusText:"Service Unavailable",headers:new Headers({"Content-Type":"text/html"})})}}))}),self.addEventListener("activate",function(t){t.waitUntil(caches.keys().then(function(t){return Promise.all(t.filter(function(t){return!t.startsWith(e)}).map(function(e){return caches.delete(e)}))}).then(function(){}))});