import { Component, OnInit } from "@angular/core";
import {
  FormBuilder,
  FormGroup,
  FormControl,
  Validators
} from "@angular/forms";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { Router, ActivatedRoute } from "@angular/router";
import { Profile, GenderType, Gender, session } from "../model";

@Component({
  selector: "profile-form",
  templateUrl: "./profile-form.component.html",
  styleUrls: ["./profile-form.component.css"]
})
export class ProfileFormComponent implements OnInit {
  profile: FormGroup;
  genders: Gender[];

  constructor(private fb: FormBuilder, private http: HttpClient) {
    this.profile = fb.group({
      name: ["", [Validators.required]],
      gender_id: [1, [Validators.required]],
      birthday: [new Date(), [Validators.required]],
      introduction: ["", [Validators.required]]
    });
  }

  queryGenders() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    this.http.get("/api/v1/genders", options).subscribe((response: any) => {
      this.genders = response;
    });
  }

  queryProfile() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json",
      Authorization: "Bearer " + window.localStorage.getItem("jwt")
    });
    let options = {
      headers: headers
    };

    let user = session.currUser();
    let apiUri = "/api/v1/users/" + user.id + "/profile";

    this.http.get(apiUri, options).subscribe((response: Profile) => {
      this.profile.controls["name"].setValue(response.name);
      this.profile.controls["gender_id"].setValue(response.gender.id);
      this.profile.controls["birthday"].setValue(new Date(response.birthday));
      this.profile.controls["introduction"].setValue(response.introduction);
    });
  }

  ngOnInit() {
    this.queryGenders();
    this.queryProfile();
  }

  update({ value, valid }: { value: any; valid: boolean }) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json",
      Authorization: "Bearer " + window.localStorage.getItem("jwt")
    });
    let options = {
      headers: headers
    };

    let user = session.currUser();
    let apiUri = "/api/v1/users/" + user.id + "/profile";
    let params = {
      name: value.name,
      gender_id: +value.gender_id,
      birthday: new Date(value.birthday).toISOString(),
      introduction: value.introduction
    };
    this.http.post(apiUri, params, options).subscribe((response: Response) => {
      console.log(response);
    });
  }
}
