export interface Bin {
    id: number;
    key: string;
    name: string;
    description: string;
    filename: string;
    content: number[];
    language: string;
    views: number,
    created_at: string;
}

export interface Statistics {
    total_bins: number;
    total_lines: number;
    total_size_mb: number;
    total_views: number;
}

const BASE_PATH = "http://localhost:3000/";

export const getBinByKey = (key: string, fetchMethod: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response> = fetch): Promise<Bin> =>
    fetchMethod(BASE_PATH+"bin/"+key).then(res => res.json());

export const createBin = async (name: string, description: string, filename: string, content: string, language: string, fetchMethod: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response> = fetch): Promise<Bin> => {
    const res = await fetchMethod(BASE_PATH+"bin", {
        method: "POST",
        body: JSON.stringify({
            name,
            description,
            filename,
            content: Array.from(new TextEncoder().encode(content)),
            language: language,
        }),
        headers: {
            "content-type": "application/json"
        }
    });
    return await res.json();
}

export const getStatistics = async (fetchMethod: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response> = fetch): Promise<Statistics> => {
    const res = await fetchMethod(BASE_PATH+"statistics");
    return await res.json();
}