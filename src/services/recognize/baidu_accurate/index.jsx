import { fetch } from '@tauri-apps/plugin-http';

export async function recognize(base64, language, options = {}) {
    const { config } = options;

    const { client_id, client_secret } = config;

    const url = 'https://aip.baidubce.com/rest/2.0/ocr/v1/accurate_basic';
    const token_url = 'https://aip.baidubce.com/oauth/2.0/token';

    const tokenApiUrl = new URL(token_url);
    tokenApiUrl.searchParams.set('grant_type', 'client_credentials');
    tokenApiUrl.searchParams.set('client_id', client_id);
    tokenApiUrl.searchParams.set('client_secret', client_secret);

    const token_res = await fetch(tokenApiUrl.toString(), {
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

            const ocrApiUrl = new URL(url);
            ocrApiUrl.searchParams.set('access_token', token);

            const res = await fetch(ocrApiUrl.toString(), {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                },
                body: new URLSearchParams({
                    language_type: language,
                    detect_direction: 'false',
                    image: base64,
                }),
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
                const errorData = await res.json();
                throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(errorData)}`;
            }
        } else {
            throw 'Get Access Token Failed!';
        }
    } else {
        const errorData = await token_res.json();
        throw `Http Request Error\nHttp Status: ${token_res.status}\n${JSON.stringify(errorData)}`;
    }
}

export * from './Config';
export * from './info';
