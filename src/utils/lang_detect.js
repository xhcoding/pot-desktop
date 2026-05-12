import { fetch } from '@tauri-apps/plugin-http';
import { invoke } from '@tauri-apps/api/core';
import { store } from './store';
import { v4 as uuidv4 } from 'uuid';

// https://fanyi-api.baidu.com/product/113
async function baidu_detect(text) {
    const lang_map = {
        zh: 'zh_cn',
        cht: 'zh_tw',
        en: 'en',
        jp: 'ja',
        kor: 'ko',
        fra: 'fr',
        spa: 'es',
        ru: 'ru',
        de: 'de',
        it: 'it',
        tr: 'tr',
        pt: 'pt_pt',
        vie: 'vi',
        id: 'id',
        th: 'th',
        may: 'ms',
        ar: 'ar',
        hi: 'hi',
        nob: 'nb_no',
        nno: 'nn_no',
        per: 'fa',
        ukr: 'uk'
    };
    const params = new URLSearchParams();
    params.append('query', text);
    let res = await fetch('https://fanyi.baidu.com/langdetect', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: params,
    });
    if (res.ok) {
        let result = await res.json();
        if (result.lan && result.lan in lang_map) {
            return lang_map[result.lan];
        }
    }
    return 'en';
}
// 腾讯只支持这么多语言
// https://cloud.tencent.com/document/product/551/15619
async function tencent_detect(text) {

    const lang_map = {
        zh: 'zh_cn',
        en: 'en',
        ja: 'ja',
        ko: 'ko',
        fr: 'fr',
        es: 'es',
        ru: 'ru',
        de: 'de',
        it: 'it',
        tr: 'tr',
        pt: 'pt_pt',
        vi: 'vi',
        id: 'id',
        th: 'th',
        ms: 'ms',
        ar: 'ar',
        hi: 'hi',
    };
    const params = new URLSearchParams();
    params.append('sourceText', text);
    let res = await fetch('https://fanyi.qq.com/api/translate', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: params,
    });
    if (res.ok) {
        let result = await res.json();
        if (result.translate && result.translate.source && result.translate.source in lang_map) {
            return lang_map[result.translate.source];
        }
    }
    return 'en';
}
// https://cloud.google.com/translate/docs/languages?hl=zh-cn
async function google_detect(text) {
    const lang_map = {
        'zh-CN': 'zh_cn',
        'zh-TW': 'zh_tw',
        ja: 'ja',
        en: 'en',
        ko: 'ko',
        fr: 'fr',
        es: 'es',
        ru: 'ru',
        de: 'de',
        it: 'it',
        tr: 'tr',
        pt: 'pt_pt',
        vi: 'vi',
        id: 'id',
        th: 'th',
        ms: 'ms',
        ar: 'ar',
        hi: 'hi',
        mn: 'mn_cy',
        km: 'km',
        fa: 'fa',
        no: 'nb_no',
        uk: 'uk'
    };
    const url = new URL('https://translate.google.com/translate_a/single');
    url.searchParams.set('dt', 'at');
    url.searchParams.set('dt', 'bd');
    url.searchParams.set('dt', 'ex');
    url.searchParams.set('dt', 'ld');
    url.searchParams.set('dt', 'md');
    url.searchParams.set('dt', 'qca');
    url.searchParams.set('dt', 'rw');
    url.searchParams.set('dt', 'rm');
    url.searchParams.set('dt', 'ss');
    url.searchParams.set('dt', 't');
    url.searchParams.set('client', 'gtx');
    url.searchParams.set('sl', 'auto');
    url.searchParams.set('tl', 'zh-CN');
    url.searchParams.set('hl', 'zh-CN');
    url.searchParams.set('ie', 'UTF-8');
    url.searchParams.set('oe', 'UTF-8');
    url.searchParams.set('otf', '1');
    url.searchParams.set('ssel', '0');
    url.searchParams.set('tsel', '0');
    url.searchParams.set('kc', '7');
    url.searchParams.set('q', text);
    let res = await fetch(url.toString(), {
        method: 'GET',
        headers: { 'content-type': 'application/json' },
    });
    if (res.ok) {
        const result = await res.json();
        if (result[2] && result[2] in lang_map) {
            return lang_map[result[2]];
        }
    }
    return 'en';
}
// https://niutrans.com/documents/contents/trans_text#languageList
async function niutrans_detect(text) {
    const lang_map = {
        zh: 'zh_cn',
        cht: 'zh_cn',
        en: 'en',
        ja: 'ja',
        ko: 'ko',
        fr: 'fr',
        es: 'es',
        ru: 'ru',
        de: 'de',
        it: 'it',
        tr: 'tr',
        pt: 'pt_pt',
        vi: 'vi',
        id: 'id',
        th: 'th',
        ms: 'ms',
        ar: 'ar',
        hi: 'hi',
        mn: 'mn_cy',
        mo: 'mn_mo',
        km: 'km',
        nb: 'nb_no',
        nn: 'nn_no',
        fa: 'fa',
        uk: 'uk'
    };
    const url = new URL('https://test.niutrans.com/NiuTransServer/language');
    url.searchParams.set('src_text', text);
    url.searchParams.set('source', 'text');
    url.searchParams.set('time', String(new Date().getTime()));
    let res = await fetch(url.toString(), {
        method: 'GET',
        headers: { 'content-type': 'application/json' },
    });
    if (res.ok) {
        const result = await res.json();
        if (result['language'] && result['language'] in lang_map) {
            return lang_map[result['language']];
        }
    }
    return 'en';
}
// https://yandex.com/dev/translate/doc/en/concepts/api-overview
async function yandex_detect(text) {
    const lang_map = {
        zh: 'zh_cn',
        en: 'en',
        ja: 'ja',
        ko: 'ko',
        fr: 'fr',
        es: 'es',
        ru: 'ru',
        de: 'de',
        it: 'it',
        tr: 'tr',
        pt: 'pt_pt',
        vi: 'vi',
        id: 'id',
        th: 'th',
        ms: 'ms',
        ar: 'ar',
        hi: 'hi',
        no: 'nb_no',
        fa: 'fa',
        uk: 'uk'
    };
    const url = new URL('https://translate.yandex.net/api/v1/tr.json/detect');
    url.searchParams.set('id', uuidv4().replaceAll('-', '') + '-0-0');
    url.searchParams.set('srv', 'android');
    url.searchParams.set('text', text);
    let res = await fetch(url.toString(), {
        method: 'GET',
    });
    if (res.ok) {
        const result = await res.json();
        if (result['lang'] && result['lang'] in lang_map) {
            return lang_map[result['lang']];
        }
    }
    return 'en';
}
// https://learn.microsoft.com/en-us/azure/ai-services/translator/language-support
async function bing_detect(text) {
    const lang_map = {
        'zh-Hans': 'zh_cn',
        'zh-Hant': 'zh_tw',
        en: 'en',
        ja: 'ja',
        ko: 'ko',
        fr: 'fr',
        es: 'es',
        ru: 'ru',
        de: 'de',
        it: 'it',
        tr: 'tr',
        'pt-pt': 'pt_pt',
        pt: 'pt_br',
        vi: 'vi',
        id: 'id',
        th: 'th',
        ms: 'ms',
        ar: 'ar',
        hi: 'hi',
        'mn-Cyrl': 'mn_cy',
        'mn-Mong': 'mn_mo',
        km: 'km',
        nb: 'nb_no',
        fa: 'fa',
        uk: 'uk'
    };
    const token_url = 'https://edge.microsoft.com/translate/auth';

    let token = await fetch(token_url, {
        method: 'GET',
        headers: {
            'User-Agent':
                'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.42',
        },
    });
    if (token.ok) {
        const tokenText = await token.text();
        const url = new URL('https://api-edge.cognitive.microsofttranslator.com/detect');
        url.searchParams.set('api-version', '3.0');

        let res = await fetch(url.toString(), {
            method: 'POST',
            headers: {
                accept: '*/*',
                'accept-language': 'zh-TW,zh;q=0.9,ja;q=0.8,zh-CN;q=0.7,en-US;q=0.6,en;q=0.5',
                authorization: 'Bearer ' + tokenText,
                'cache-control': 'no-cache',
                'content-type': 'application/json',
                pragma: 'no-cache',
                'sec-ch-ua': '"Microsoft Edge";v="113", "Chromium";v="113", "Not-A.Brand";v="24"',
                'sec-ch-ua-mobile': '?0',
                'sec-ch-ua-platform': '"Windows"',
                'sec-fetch-dest': 'empty',
                'sec-fetch-mode': 'cors',
                'sec-fetch-site': 'cross-site',
                Referer: 'https://appsumo.com/',
                'Referrer-Policy': 'strict-origin-when-cross-origin',
                'User-Agent':
                    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.42',
            },
            body: JSON.stringify([{ Text: text }]),
        });

        if (res.ok) {
            let result = await res.json();
            if (result[0].language && result[0].language in lang_map) {
                return lang_map[result[0].language];
            }
        }
    }
    return 'en';
}

async function local_detect(text) {
    return await invoke('lang_detect', { text: text });
}

export default async function detect(text) {
    let langDetectEngine = (await store.get('translate_detect_engine')) ?? 'baidu';

    switch (langDetectEngine) {
        case 'baidu':
            return await baidu_detect(text);
        case 'google':
            return await google_detect(text);
        case 'local':
            return await local_detect(text);
        case 'tencent':
            return await tencent_detect(text);
        case 'niutrans':
            return await niutrans_detect(text);
        case 'yandex':
            return await yandex_detect(text);
        case 'bing':
            return await bing_detect(text);
        default:
            return await local_detect(text);
    }
}
