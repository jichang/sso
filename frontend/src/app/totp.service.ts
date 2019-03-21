import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { session } from "./model";

export interface QrCodeConfig {
  size: number;
  modules: boolean[][];
}

export interface TotpStore {
  qrcodeConfig?: QrCodeConfig;
}

@Injectable({
  providedIn: "root"
})
export class TotpService {
  store: TotpStore = {
    qrcodeConfig: {
      size: 0,
      modules: []
    }
  };
  private subject: BehaviorSubject<TotpStore> = new BehaviorSubject(null);

  constructor(private http: HttpClient) {}

  get qrcode() {
    return this.subject.asObservable();
  }

  select() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = "/api/v1/users/" + session.currUser().id + "/totp/qrcode";
    this.http.get(apiUri, options).subscribe((qrcode: QrCodeConfig) => {
      this.store.qrcodeConfig = qrcode;
      this.subject.next(this.store);
    });
  }

  update(params: { code: number }) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = "/api/v1/users/" + session.currUser().id + "/totp";
    return this.http.post(apiUri, params, options);
  }
}
