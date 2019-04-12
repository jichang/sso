import { Component, OnInit, Output, EventEmitter } from "@angular/core";
import { FormBuilder, FormGroup, Validators } from "@angular/forms";
import { HttpClient } from "@angular/common/http";

interface SignupParams {
  username: string;
  password: string;
  invitation_code: string;
}

@Component({
  selector: "signup-form",
  templateUrl: "./signup-form.component.html",
  styleUrls: ["./signup-form.component.css"]
})
export class SignupFormComponent implements OnInit {
  user: FormGroup;
  @Output() success = new EventEmitter();
  @Output() failure = new EventEmitter();

  constructor(private fb: FormBuilder, private http: HttpClient) {
    this.user = fb.group({
      username: [
        "",
        [Validators.compose([Validators.required, Validators.minLength(1)])]
      ],
      password: [
        "",
        [Validators.compose([Validators.required, Validators.minLength(1)])]
      ],
      invitation_code: [
        "",
        [Validators.compose([Validators.required, Validators.minLength(1)])]
      ]
    });
  }

  ngOnInit() {}

  signup({ value, valid }: { value: SignupParams; valid: boolean }) {
    let params = {
      username: value.username,
      password: value.password,
      invitation_code: value.invitation_code
    };

    this.http.post("/api/v1/signup", params).subscribe(
      (response: Response) => {
        this.success.emit(response);
      },
      (response: Response) => {
        this.failure.emit(response);
      }
    );
  }
}
