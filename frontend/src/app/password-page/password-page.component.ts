import { Component, OnInit } from "@angular/core";
import { MatSnackBar } from "@angular/material/snack-bar";

@Component({
  selector: "password-page",
  templateUrl: "./password-page.component.html",
  styleUrls: ["./password-page.component.css"]
})
export class PasswordPageComponent implements OnInit {
  constructor(private snackBar: MatSnackBar) {}

  ngOnInit() {}

  onPasswordUpdateSuccess(evt) {
    this.snackBar.open("Password update success", "Dismiss", {
      duration: 3000
    });
  }

  onPasswordUpdateFailure(evt) {
    this.snackBar.open("Password update failure", "Dismiss", {
      duration: 3000
    });
  }
}
