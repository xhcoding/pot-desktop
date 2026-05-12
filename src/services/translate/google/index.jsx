import { fetch } from '@tauri-apps/plugin-http';

export async function translate(text, from, to, options = {}) {
    const { config } = options;

    let { custom_url } = config;

    if (custom_url === undefined || custom_url === '') {
        custom_url = 'https://translate.google.com';
    }
    if (!custom_url.startsWith('http')) {
        custom_url = 'https://' + custom_url;
    }

    const apiUrl = new URL(`${custom_url}/translate_a/single`);
    apiUrl.searchParams.set('dt', 'at');
    apiUrl.searchParams.set('dt', 'bd');
    apiUrl.searchParams.set('dt', 'ex');
    apiUrl.searchParams.set('dt', 'ld');
    apiUrl.searchParams.set('dt', 'md');
    apiUrl.searchParams.set('dt', 'qca');
    apiUrl.searchParams.set('dt', 'rw');
    apiUrl.searchParams.set('dt', 'rm');
    apiUrl.searchParams.set('dt', 'ss');
    apiUrl.searchParams.set('dt', 't');
    apiUrl.searchParams.set('client', 'gtx');
    apiUrl.searchParams.set('sl', from);
    apiUrl.searchParams.set('tl', to);
    apiUrl.searchParams.set('hl', to);
    apiUrl.searchParams.set('ie', 'UTF-8');
    apiUrl.searchParams.set('oe', 'UTF-8');
    apiUrl.searchParams.set('otf', '1');
    apiUrl.searchParams.set('ssel', '0');
    apiUrl.searchParams.set('tsel', '0');
    apiUrl.searchParams.set('kc', '7');
    apiUrl.searchParams.set('q', text);

    let res = await fetch(apiUrl.toString(), {
        method: 'GET',
        headers: { 'content-type': 'application/json' },
    });
    if (res.ok) {
        let result = await res.json();
        // 词典模式
        if (result[1]) {
            let target = { pronunciations: [], explanations: [], associations: [], sentence: [] };
            // 发音
            if (result[0][1][3]) {
                target.pronunciations.push({ symbol: result[0][1][3], voice: '' });
            }
            // 释义
            for (let i of result[1]) {
                target.explanations.push({
                    trait: i[0],
                    explains: i[2].map((x) => {
                        return x[0];
                    }),
                });
            }
            // 例句
            if (result[13]) {
                for (let i of result[13][0]) {
                    target.sentence.push({ source: i[0] });
                }
            }
            return target;
        } else {
            // 翻译模式
            let target = '';
            for (let r of result[0]) {
                if (r[0]) {
                    target = target + r[0];
                }
            }
            return target.trim();
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(await res.json())}`;
    }
}

export * from './Config';
export * from './info';
