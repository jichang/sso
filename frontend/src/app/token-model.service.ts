import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders, HttpParams } from "@angular/common/http";
import { map } from "rxjs/operators";
import { session } from "./model";

export interface CreateParams {
  target_id: number;
  target_type: "email";
  target_identity: string;
  action: "verify";
}

export interface DeleteParams {
  target_id: number;
  target_type: "email";
  action: "verify";
  token: string;
}

@Injectable({
  providedIn: "root"
})
export class TokenModelService {
  constructor(private http: HttpClient) {}

  create(params: CreateParams) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = "/api/v1/users/" + session.currUser().id + "/tokens";
    return this.http.post(apiUri, params, options);
  }

  remove(params: DeleteParams) {
    let httpParams = new HttpParams()
      .set("target_id", params.target_id.toString())
      .set("target_type", params.target_type)
      .set("action", "verify")
      .set("token", params.token);

    let apiUri = `/api/v1/users/${session.currUser().id}/tokens`;

    return this.http.delete(apiUri, {
      params: httpParams
    });
  }
}
