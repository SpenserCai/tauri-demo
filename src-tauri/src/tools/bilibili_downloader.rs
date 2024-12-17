use core::result::Result;

/*
 * @Author: SpenserCai
 * @Date: 2024-12-16 15:56:17
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-17 14:02:31
 * @Description: file content
 */
// use env_logger::Env;
use reqwest::blocking::Client;
// use reqwest::header::{REFERER, USER_AGENT};
use serde_json::{from_str, Value};
use log::{info};

use crate::tools::bilibili_wbi;
// use std::{env, fs, io::Write, process::exit};
// use std::{fs,env, io::Write};

// #[derive(Debug)]
// struct Bilibili {
//     cli: Client,
//     cid_url: String,
//     cid: String,
//     play_url: String,
//     url: String,
//     real_url: String,
//     bvid: String,
//     title: String,
// }
pub struct Bilibili {
    // pub url: String,
    // pub api_playurl: String,
    // pub api_view: String,
    pub real_url: String,
    pub cli: Client,
    pub cid_url: String,
    pub cid: String,
    pub aid: String,
    pub play_url: String,
    // pub bvid: String,
    pub title: String,
    pub play_info: Value,
}

impl Bilibili {
    pub fn new(url: String, play_url: String, mut cid_base_url: String) -> Self {
        // env_logger::init();
        info!("Creating Bilibili instance for URL: {}", url);
        let mut bvid = "".to_string();
        if let Some(bv_index) = url.find("BV") {
            if url.len() >= bv_index + 12 {
                bvid.push_str(&url[bv_index..bv_index + 12]);
                cid_base_url.push_str(&url[bv_index..bv_index + 12]);
            }
        }
        Self {
            cli: Client::new(),
            cid: "".to_string(),
            aid: "".to_string(),
            cid_url: cid_base_url,
            play_url: play_url,
            // url: url,
            real_url: "".to_string(),
            // bvid: bvid,
            title: "".to_string(),
            play_info: Value::Null,
        }
    }

    // check is_bilibili_ok 如果错误（code != 0）则返回错误信息否则则是ok（染回可以用Result）
    fn is_bilibili_ok(&self, json: &Value) -> Result<(), Box<dyn std::error::Error>> {
        // 先判断是否为空
        if json.is_null() {
            return Err("Response is null".into());
        }
        let code = json["code"].as_i64().unwrap();
        if code != 0 {
            let message = json["message"].as_str().unwrap();
            return Err(message.into());
        }
        Ok(())
    }

    pub fn get_cid(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Getting CID for URL: {}", self.cid_url.clone());
        let r = self.cli.get(&self.cid_url).send()?;
        let json: Value = from_str(&r.text()?).unwrap();
        if let Err(e) = self.is_bilibili_ok(&json) {
            return Err(e);
        }
        // println!("{}", serde_json::to_string_pretty(&json).unwrap());
        let title = json["data"]["title"].as_str().unwrap();
        let cid = json["data"]["cid"].as_u64().unwrap().to_string();
        let aid = json["data"]["aid"].as_u64().unwrap().to_string();
        self.title = title.to_string();
        self.cid = cid;
        self.aid = aid;
        Ok(())
    }

    pub fn get_play_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let keys = bilibili_wbi::get_wbi_keys(None).unwrap();
        info!("Keys: {:?}", keys);
        let params = vec![
            ("avid", self.aid.clone()), 
            ("cid", self.cid.clone()),
            ("qn", "64".to_string()),
            ("platform", "html5".to_string())
        ];
        let query = bilibili_wbi::encode_wbi(params, keys);
        info!("Query: {}", query);
        self.play_url.push_str("?");
        self.play_url.push_str(&query);
        info!("Play URL: {}", self.play_url);
        // self.play_url.push_str("&fnver=0");
        // self.play_url.push_str("&fnval=1");
        // self.play_url.push_str("&fourk=1");
        let r = self.cli
            .get(&self.play_url)
            .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
            .header("Referer", "https://www.bilibili.com/")
            .send()?;
        let json: Value = from_str(&r.text()?).unwrap();
        // 判断是否为空
        if json.is_null() {
            return Err("Play info is null".into());
        }
        self.play_info = json;
        Ok(())
    }

    pub fn get_real_url(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let is_ok = self.get_play_info().is_ok();
        if !is_ok {
            return Err("Get play info failed".into());
        }
        let json = &self.play_info;
        let durl = json["data"]["durl"][0]["url"].as_str().unwrap();
        self.real_url = durl.to_string();
        Ok(())

    }

    // fn download(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    //     let _ = self.get_cid();
    //     let _ = self.get_real_url();
    //     info!("正在下载 {}，请稍后。。。", self.title);
    //     let r = self.cli.get(&self.real_url)
    //         .header(REFERER, "https://www.bilibili.com")
    //         .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:78.0) Gecko/20100101 Firefox/78.0")
    //         .send()?
    //         .bytes()?;
    //     let mut fname = self.title.clone();
    //     fname.push_str(".flv");
    //     let mut file = fs::File::create(&fname)?;
    //     file.write_all(&r)?;
    //     info!("{} 下载完成", fname);
    //     Ok(())
    // }
}