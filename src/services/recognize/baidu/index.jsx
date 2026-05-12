import { fetch } from '@tauri-apps/plugin-http';

export async function recognize(base64, language, options = {}) {
    const { config } = options;

    const { client_id, client_secret } = config;

    const url = new URL('https://aip.baidubce.com/rest/2.0/ocr/v1/general_basic');
    const token_url = new URL('https://aip.baidubce.com/oauth/2.0/token');

    token_url.searchParams.set('grant_type', 'client_credentials');
    token_url.searchParams.set('client_id', client_id);
    token_url.searchParams.set('client_secret', client_secret);

    const token_res = await fetch(token_url.toString(), {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            Accept: 'application/json',
        },
    });
    if (token_res.ok) {
        const token_data = await token_res.json();
        if (token_data.access_token) {
            let token = token_data.access_token;

            url.searchParams.set('access_token', token);
            
            const form = new URLSearchParams();
            form.append('language_type', language);
            form.append('detect_direction', 'false');
            form.append('image', base64);

            const res = await fetch(url.toString(), {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                },
                body: form,
            });
            if (res.ok) {
                let result = await res.json();
                if (result['words_result']) {
                    let target = '';
                    for (let i of result['words_result']) {
                        target += i['words'] + '\n';
                    }
                    return target.trim();
                } else {
                    throw JSON.stringify(result);
                }
            } else {
                const resData = await res.json();
                throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(resData)}`;
            }
        } else {
            throw 'Get Access Token Failed!';
        }
    } else {
        const tokenData = await token_res.json();
        throw `Http Request Error\nHttp Status: ${token_res.status}\n${JSON.stringify(tokenData)}`;
    }
}

export * from './Config';
export * from './info';
