import { Component, OnInit } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import { MatSnackBar } from "@angular/material";

@Component({
  selector: "application-create-page",
  templateUrl: "./application-create-page.component.html",
  styleUrls: ["./application-create-page.component.css"]
})
export class ApplicationCreatePageComponent implements OnInit {
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private snackBar: MatSnackBar
  ) {}

  ngOnInit() {}

  created() {
    this.router.navigate(["../"], { relativeTo: this.route });
  }

  failed() {
    this.snackBar.open("application create failure", "Dismiss", {
      duration: 3000
    });
  }
}
