async function translate(text, from, to, options) {
    const { utils } = options;
    const { tauriFetch: fetch } = utils;

    const URL = "https://translate.volcengine.com/crx/translate/v1";

    const body = {
        source_language: from,
        target_language: to,
        text
    }
    const headers = {
        'content-type': 'application/json'
    };
    let res = await fetch(URL, {
        method: 'POST',
        headers: headers,
        body: {
            type: 'Json',
            payload: body
        },
    });

    if (res.ok) {
        let result = res.data;
        const { translation } = result;
        if (translation) {
            return translation;
        } else {
            throw JSON.stringify(result.trim());
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}
